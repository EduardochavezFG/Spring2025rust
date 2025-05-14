use std::{
    env,    // For accessing command-line arguments
    fs,     // For reading from and writing to files
    sync::{Arc, Mutex, mpsc::{self, Sender, Receiver}}, // For thread-safe data sharing and communication
    thread, // For spawning worker threads
    time::{Duration, Instant, SystemTime}, // For time measurement and timestamps
};

use once_cell::sync::Lazy; // For global lazy-initialized static data
use reqwest::blocking::Client; // Synchronous HTTP client from the only allowed crate

// Struct to hold the result of checking one website
#[derive(Debug, Clone)]
struct WebsiteStatus {
    url: String,                         // The website's URL
    action_status: Result<u16, String>, // HTTP status code or error description
    response_time: Duration,            // Time taken to get a response
    timestamp: SystemTime,              // When the check was made
}

// A global thread-safe log of all website statuses
static STATUS_LOG: Lazy<Arc<Mutex<Vec<WebsiteStatus>>>> =
    Lazy::new(|| Arc::new(Mutex::new(Vec::new())));

// Serialize all website statuses into JSON and write them to a file
fn log_statuses_to_file(statuses: &[WebsiteStatus]) {
    let output = statuses.iter().map(|s| {
        let mut map = std::collections::BTreeMap::new();
        map.insert("url", s.url.clone());
        map.insert(
            "status",
            match &s.action_status {
                Ok(code) => code.to_string(),
                Err(err) => err.clone(),
            },
        );
        map.insert("response_time_ms", s.response_time.as_millis().to_string());
        map.insert("timestamp", format!("{:?}", s.timestamp));
        map
    }).collect::<Vec<_>>();

    // Serialize to pretty JSON format and write to file
    let json = serde_json::to_string_pretty(&output).expect("Failed to serialize");
    fs::write("website_statuses.json", json).expect("Failed to write file");
}

// Try to fetch a website with retry logic and a timeout
fn fetch_website(client: &Client, url: &str, timeout: Duration, retries: usize) -> WebsiteStatus {
    let start = Instant::now();         // Start timing the request
    let timestamp = SystemTime::now();  // Record when the attempt started
    let mut attempts = 0;

    loop {
        let result = client.get(url).timeout(timeout).send(); // Attempt to send HTTP GET request
        match result {
            Ok(resp) => {
                return WebsiteStatus {
                    url: url.to_string(),
                    action_status: Ok(resp.status().as_u16()),
                    response_time: start.elapsed(),
                    timestamp,
                };
            }
            Err(err) => {
                attempts += 1;
                if attempts > retries {
                    // Give up and return error result after max retries
                    return WebsiteStatus {
                        url: url.to_string(),
                        action_status: Err(err.to_string()),
                        response_time: start.elapsed(),
                        timestamp,
                    };
                }
                thread::sleep(Duration::from_millis(100)); // Wait before retrying
            }
        }
    }
}

// Worker thread function that listens on the job queue and checks websites
fn worker(
    id: usize,
    rx: Arc<Mutex<Receiver<String>>>,
    log: Arc<Mutex<Vec<WebsiteStatus>>>,
    client: Client,
    timeout: Duration,
    retries: usize,
) {
    loop {
        // Receive the next URL to check from the queue
        let url = {
            let lock = rx.lock().unwrap();
            lock.recv()
        };

        match url {
            Ok(url) => {
                println!("Worker {id} checking {url}"); // Live output
                let status = fetch_website(&client, &url, timeout, retries); // Perform check
                {
                    let mut log_lock = log.lock().unwrap(); // Store the result in shared log
                    log_lock.push(status);
                }
            }
            Err(_) => break, // Exit if the channel is closed
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect(); // Read CLI arguments
    if args.len() == 1 {
        // Show help if no arguments are given
        eprintln!("Usage: website_checker [--file path] [URL ...] [--workers N] [--timeout S] [--retries N]");
        std::process::exit(2);
    }

    let mut urls = Vec::new();            // List of URLs to check
    let mut workers = num_cpus::get();    // Default: number of logical CPUs
    let mut timeout = 5;                  // Default timeout in seconds
    let mut retries = 0;                  // Default: no retries
    let mut i = 1;

    // Parse command-line arguments
    while i < args.len() {
        match args[i].as_str() {
            "--file" => {
                i += 1;
                if i < args.len() {
                    // Read URLs from a file (ignoring blanks/comments)
                    let content = fs::read_to_string(&args[i]).expect("Failed to read file");
                    for line in content.lines() {
                        let line = line.trim();
                        if !line.is_empty() && !line.starts_with('#') {
                            urls.push(line.to_string());
                        }
                    }
                }
            }
            "--workers" => {
                i += 1;
                if i < args.len() {
                    workers = args[i].parse().unwrap_or(workers);
                }
            }
            "--timeout" => {
                i += 1;
                if i < args.len() {
                    timeout = args[i].parse().unwrap_or(timeout);
                }
            }
            "--retries" => {
                i += 1;
                if i < args.len() {
                    retries = args[i].parse().unwrap_or(retries);
                }
            }
            val => {
                // Any non-flag arguments are assumed to be URLs
                if !val.starts_with("--") {
                    urls.push(val.to_string());
                }
            }
        }
        i += 1;
    }

    // Fail if no URLs were provided
    if urls.is_empty() {
        eprintln!("No URLs provided.");
        std::process::exit(2);
    }

    // Set up a channel for sending URLs to worker threads
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));
    let log = Arc::clone(&STATUS_LOG);
    let client = Client::builder().build().expect("Failed to build client");
    let timeout_duration = Duration::from_secs(timeout as u64);

    // Spawn the fixed number of worker threads
    for i in 0..workers {
        let thread_rx = Arc::clone(&rx);
        let thread_log = Arc::clone(&log);
        let thread_client = client.clone();
        let thread_timeout = timeout_duration;
        thread::spawn(move || {
            worker(i, thread_rx, thread_log, thread_client, thread_timeout, retries)
        });
    }

    // Send all URLs into the job queue
    for url in urls {
        tx.send(url).unwrap();
    }

    drop(tx); // Close the sending end to signal workers to exit when done
    thread::sleep(Duration::from_secs(2)); // Allow threads time to finish

    // Write results to output JSON file
    let final_log = STATUS_LOG.lock().unwrap();
    log_statuses_to_file(&final_log);
    println!("Statuses written to website_statuses.json");
}

