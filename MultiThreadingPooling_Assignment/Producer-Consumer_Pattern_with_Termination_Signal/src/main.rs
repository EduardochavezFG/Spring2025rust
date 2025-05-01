//Assignment 4: Producer-Consumer Pattern with Termination Signal

use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use rand::Rng;

// ========== THREAD POOL IMPLEMENTATION ==========

enum Message {
    NewJob(Job),
    Terminate,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending terminate messages to all workers...");

        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

// ========== PRODUCER-CONSUMER IMPLEMENTATION ==========

const TERMINATION_SIGNAL: i32 = -1;
const ITEM_COUNT: usize = 20;

fn producer(id: usize, tx: mpsc::Sender<i32>, item_count: usize) {
    let mut rng = rand::thread_rng();

    for _ in 0..item_count {
        let num = rng.gen_range(1..=100);
        println!("Producer {} produced {}", id, num);
        tx.send(num).unwrap();
        thread::sleep(Duration::from_millis(100));
    }
}

fn consumer(id: usize, rx: Arc<Mutex<mpsc::Receiver<i32>>>) {
    loop {
        let num = rx.lock().unwrap().recv().unwrap();
        if num == TERMINATION_SIGNAL {
            println!("Consumer {} received termination signal.", id);
            break;
        }
        println!("Consumer {} consumed {}", id, num);
        thread::sleep(Duration::from_millis(150));
    }
}

fn main() {
    println!("=== Thread Pool Execution ===");
    let pool = ThreadPool::new(4);

    for i in 1..=10 {
        pool.execute(move || {
            println!("Processing task {}", i);
            thread::sleep(Duration::from_millis(500));
            println!("Completed task {}", i);
        });
    }

    println!("Main thread waiting for thread pool tasks to complete...");
    // ThreadPool drops here and joins workers

    println!("\n=== Producer-Consumer Execution ===");

    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));

    let mut handles = vec![];

    // 2 producers
    for id in 0..2 {
        let tx_clone = tx.clone();
        handles.push(thread::spawn(move || {
            producer(id, tx_clone, ITEM_COUNT / 2);
        }));
    }

    // 3 consumers
    for id in 0..3 {
        let rx_clone = Arc::clone(&rx);
        handles.push(thread::spawn(move || {
            consumer(id, rx_clone);
        }));
    }

    // Wait for producers to finish
    let producer_handles: Vec<_> = handles.drain(0..2).collect();
    for handle in producer_handles {
        handle.join().unwrap();
    }

    // Send termination signals
    for _ in 0..3 {
        tx.send(TERMINATION_SIGNAL).unwrap();
    }

    // Wait for consumers to finish
    for handle in handles {
        handle.join().unwrap();
    }

    println!("All items have been produced and consumed!");
}