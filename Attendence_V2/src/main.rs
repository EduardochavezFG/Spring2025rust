use std::fs::File;
use std::io::{self, Write, Read};

struct Config {
    name: String,
    utrgv_id: String,
}

impl Config {
    fn from_file(path: &str) -> Config {
        let mut file = File::open(path).expect("Failed to open the file.");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read the file.");

        let mut lines = contents.lines();
        let name = lines.next().expect("Missing name").to_string();
        let utrgv_id = lines.next().expect("Missing API key").to_string();
        Config { name, utrgv_id }
    }
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap(); 
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn main() {
    let name = get_user_input("Enter Name: ");
    let utrgv_id = get_user_input("Enter UTRGV ID: ");
   
    let mut file = File::create("config.txt").expect("Failed to create file.");
    writeln!(file, "{}", name).unwrap();
    writeln!(file, "{}", utrgv_id).unwrap();

    let config = Config::from_file("config.txt");
    println!("\n--- Config Loaded ---");
    println!("Name: {}", config.name);
    println!("UTRGV ID: {}", config.utrgv_id);
}
