use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Get the file path from command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <file_path>");
        std::process::exit(1);
    } else if args.len() > 2 {
        eprintln!("Too many arguments provided. Please provide only the file path.");
        std::process::exit(1);
        
    }
    // Open the file specified in the command line argument
    // Handle errors appropriately
    // If the file does not exist, panic with a message
    // If there is an error reading the file, panic with a message
    // If the file is opened successfully, read it line by line and print each line to stdout
    // If there is an error reading a line, panic with a message
    let file = File::open(&args[1]);
    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                panic!("File not found: {}", error)
            }
            _ => {
                panic!("Error opening file: {}", error)
            }
        },
    };

    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error)
            }
        }
    }
}
