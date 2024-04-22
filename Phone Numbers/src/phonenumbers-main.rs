// This program uses Regular expressions to find phone numbers in a 
// in a text file passed to it as a parameter, then outputs the results
// to the console.

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

// Clears the console of output.

fn clear_screen() {
    print!("{}[2J", 27 as char);
}


// This program accepts as input a text file and uses pattern matching with
// regular expressions to extract all the phone numbers contained, then prints
// them to the console

fn main() {

    let args: Vec<String> = env::args().collect(); // collect cli arguments

    if args.len() == 2 {

        let file = File::open(&args[1]).unwrap();
        let buffer = BufReader::new(file);        // read contents of file into buffer

                                           //------------------ NANP -------------------- | NANP Emergency | ----------------------- International -------------------------- |
        let phone_pattern = Regex::new(r#"^[+1]*[-.\( ]*\d{3}{0,1}[-.\) ]*[2-9]\d{2}[-. ]*\d{4}$|^[0-9]11$|^988$|^(?:\+|0|00)[-.\( ]*\d{1,3}[-.\( ]*\d{1,8}[-.\( ]*\d{2,10}$"#).unwrap();

        clear_screen();
        println!("---------------------\n");
        println!("Phone numbers found in {}:", args[1]);
        println!("---------------------\n");
        
        for line in buffer.lines() {
            if let Ok(line) = line {
                for phone_number in phone_pattern.find_iter(&line) {
                    println!("{}", phone_number.as_str().replace(" ", ""));
                }
            }
        }
        
    } else {
        println!{"\n---------------------------------------------------------------------"};
        println!{"\nPHONE NUMBER SEARCH is a program that finally lets you live out your"};
        println!{"dream of searching through text files to find phone numbers within."};
        println!{"To use PHONE NUMBER SEARCH, please run this program with the text"};
        println!{"file of your choice, and it will output the phone numbers contained."};
        println!{"\n---------------------------------------------------------------------\n"};
        return;
    }
    
    println!("\n----------------------------------------\n");
    println!("Thank you for using PHONE NUMBER SEARCH.");
    println!("\n----------------------------------------\n");
    
}