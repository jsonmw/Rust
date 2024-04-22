// This program uses Regular expressions to find addresses in a text
// file passed to it as a parameter, then outputs them to the console.

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

// Clears the console of output.

fn clear_screen() {
    print!("{}[2J", 27 as char);
}

// Outputs the addresses to the console

fn output_addresses(addresses: &Vec<String>, filename: &String) {
    clear_screen();
    
    println!("Addresses found in {}:\n", filename);
    for address in addresses {
        println!("\t- {}", address.trim());
    }
    println!();
}

fn main() {
    // Get the name of the input file from the command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
    let filename = &args[1];

        // Open the input file and read its contents into a string
        let file = File::open(filename).expect("Failed to open file");
        let reader = BufReader::new(file);
        let mut contents = String::new();
        for line in reader.lines() {
            contents.push_str(&line.unwrap());
            contents.push_str("\n");
        }
    
        // creates a regular expression of <number> <street>, <city>, <state>, <zip code>
        let address_pattern = Regex::new(r"\d*\s+\w+\s+\w+\.,*\s*\w*\s*\w*,*\s+\w{2},*\s+\d{5}").unwrap();
    
        let mut addresses = Vec::new();
        for cap in address_pattern.captures_iter(&contents) {
            addresses.push(cap[0].to_string());
        }
    
        output_addresses(&addresses, &filename);
    } else {
        println!{"\n---------------------------------------------------------------"};
        println!{"\nADDRESS SEARCH is a program that finally lets you live out your"};
        println!{"dream of searching through text files to find addresses within."};
        println!{"To use ADDRESS SEARCH, please run this program with the text"};
        println!{"file of your choice, and it will output the addresses contained."};
        println!{"\n---------------------------------------------------------------"};
        return;
    }

    println!("\n----------------------------------------\n");
    println!("Thank you for using ADDRESS SEARCH.");
    println!("\n----------------------------------------\n");
    
}

