// This program uses Regular expressions to find user-specified data 
// in a text file of a log passed to it as a parameter or given as
// prompted input, then outputs the results to the console.

use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

// Clears the console of output.

fn clear_screen() {
    print!("{}[2J", 27 as char);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut log_file_path = String::new();

    if args.len() == 2 {
        log_file_path = args[1].clone()
    } else {
        clear_screen();
        let mut valid = false;
        while !valid {
            println!("Which log file would you like to query? (if not in current directory, provide absolute path):\n");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let path = input.trim();
            if std::path::Path::new(&path).is_file() {
                log_file_path = path.to_string();
                valid = true;
            } else {
                println!("\nInvalid file name.\n");
            }
        }
    };

    let log_file = File::open(&log_file_path).unwrap();
    let mut log_data: Vec<String> = vec![];
    let buffer = BufReader::new(log_file);

    // read log file line by line and store in a vector
    for line in buffer.lines() {
        let log_line = line.unwrap();
        log_data.push(log_line);
    }

    clear_screen();

    println!("QUERYING {}\n", log_file_path);
    println!("Select query type:");
    println!("\n--------------------------------\n");
    println!("\t1) Date");
    println!("\t2) Severity");
    println!("\t3) ID");
    println!("\t4) Search a particular string\n");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

 
    let query_pattern; // the soon to be regex
    let mut query_input = String::new();

    match input.trim() {
        "1" => {
            println!("\nEnter date (yyyy-mm-dd):\n");
            io::stdin().read_line(&mut query_input).unwrap();
            let date = query_input.trim();
            query_pattern = Regex::new(date).unwrap();
        }
        "2" => {
            println!("\nEnter severity (WARNING | INFO | ERROR):\n");
            io::stdin().read_line(&mut query_input).unwrap();
            let severity = query_input.trim().to_uppercase();
            query_pattern = Regex::new(&format!(r"\[{}\]", severity)).unwrap();
        }
        "3" => {
            println!("\nEnter ID #:\n");
            io::stdin().read_line(&mut query_input).unwrap();
            let id = query_input.trim();
            query_pattern = Regex::new(&format!(r"\[ID: {}\]", id)).unwrap();
        }
        "4" => {
            println!("\nEnter search string:\n");
            io::stdin().read_line(&mut query_input).unwrap();
            let query = query_input.trim();
            query_pattern = Regex::new(&query).unwrap();
        }
        _ => {
            println!("Invalid query. Please reload program.");
            return;
        }
    }

    clear_screen();    
    println!("\nRESULTS:");
    println!("\n--------------------------------\n");
    
    // Output results to console
    
    for log in log_data {
        if query_pattern.is_match(&log) {
            println!("{}", log);
        }
    }
    
    println!("\n-------------------------------------\n");
    println!("Thanks for using my bargain bin grep!");
    println!("\n-------------------------------------\n");

}