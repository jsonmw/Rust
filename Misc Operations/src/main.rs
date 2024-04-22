// This program allows the user to select a particular calculation they would
// like to make, and enforces certain preconditions on the bounds of those 
// calculations, in order to prevent errors and crashing.

use std::io;
use std::str;

// Clears console screen

fn clear_screen() {
    print!("{}[2J", 27 as char);
}

// Given an unsigned integer for input and another integer representing which
// function's constraints to use as validation, this returns a boolean based
// on if the given input is valid for the given function.

fn validate_input(input: u64, function: i32) -> bool {

    match function {
        1 => {
            if input >= 0  && input <= 20 {
                return true;
            } else {
                println!{"\n\nTo prevent errors, please enter a number between 0 and 20.\n\n"}
            }
        },
        2 => {
            if input >=0 && input <= 50 {
                return true;
            } else {
                println!{"\n\nTo prevent errors, please enter a number between 0 and 50.\n\n"}
            }
        },
        3 => {
            if input >=0 && input <= 97 {
                return true;
            } else {
                println!{"\n\nTo prevent errors, please enter a number between 0 and 47.\n\n"}
            }
        },
        _ => {println!("Unrecognized function."); return false;}
    }

    false
}

// Checks if the given String contains a number

fn check_if_number(input: &String) -> bool {
    for c in input.chars() {
        if !c.is_digit(10) {
            return false;
        } 
    }
    true
}

// Calculates the factorial of the given number.
    // credit: Programming-Idioms.org ://programming-idioms.org/idiom/31/recursive-factorial-simple/450/rust

fn factorial(n: u64) -> u64 {
    match n {
        0 | 1 => 1,
        _ => factorial(n - 1) * n,
    }
}

// Presents the user with the sub-menu for the factorial function

fn factorial_menu() {
    clear_screen();
    
    let mut exit = false;

    while !exit {
        let mut valid = false;
        let mut numeric_input = 0;
        let mut input = String::new();
        let mut all_nums = true;


        while !valid {

            println!("Please enter a number you wish to calculate the factorial of\n ");
            println!("\t- enter Q to quit calculating factorials. ");
            
            input.clear();
            io::stdin().read_line(&mut input);
    
            input.pop();
            input = input.to_uppercase();
            
            if input == "Q" {
                valid = true;
                exit = true; 
            }
        
            if !valid && input.len() > 0 {

                if check_if_number(&input) {
                    let test_num = input.parse::<u64>().unwrap();
                    
                    valid = validate_input(test_num, 1);
                    
                    if valid {
                        numeric_input = test_num;
                    }
                } else {
                    println!("\n\nInvalid input: Please try again");
                    println!("--------------------------------\n\n");
                    break;
                }
            }

            if input != "Q" && input.len() !=0 && valid {
                let result = factorial(numeric_input);
                println!("\n\nThe factorial of {} is {}\n\n", numeric_input, result);
            }
        }

    }
}

// Calculates the fibonacci sequence up to the given number.
    // credit: Benjamin Brandt, https://benjaminbrandt.com/fibonacci-in-rust/

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// Presents the user with the sub-menu for the fibonacci sequence

fn fibonacci_menu() {
    clear_screen();
    
    let mut exit = false;
    
    while !exit {
        let mut valid = false;
        let mut numeric_input = 0;
        let mut input = String::new();
        let mut all_nums = true;
        
        
        while !valid {
            
            println!("Please enter a number of digits you wish to calculate the fibonacci sequence to: ");
            println!{"\tNote: Larger numbers may take more time to compute.\n"}
            println!("\t- enter Q to quit creating Fibonacci sequences. ");
            
            input.clear();
            io::stdin().read_line(&mut input);
            
            input.pop();
            input = input.to_uppercase();
            
            if input == "Q" {
                valid = true;
                exit = true; 
            }
            
            if !valid && input.len() > 0 {
                
                if check_if_number(&input) {
                    let test_num = input.parse::<u64>().unwrap();
                    
                    valid = validate_input(test_num, 2);
    
                    if valid {
                        numeric_input = test_num;
                    }
                } else {
                    println!("\n\nInvalid input: Please try again");
                    println!("--------------------------------\n\n");
                    break;
                }
            }

            if input != "Q" && valid {
                let result = fibonacci(numeric_input);
                println!("\n\nThe fibonacci sequence to {} digits is: {}\n\n", numeric_input, result);
            }
        }

    }
}


// Uses Bailey-Borwein-Plouffe algorithm to calculate pi to the given precision.
// Formats output based on if the user selected a particular digit (which is 
// initialized higher than this function's preconditions will allow for input)

fn digits_of_pi(precision: u64, digit: u64) -> String {
    
    let mut pi = 0.0;

    for i in 0..precision {
        pi += (1.0 / 16.0_f64.powi(i as i32)) * 
            ((4.0 / (8.0 * i as f64 + 1.0)) - (2.0 / (8.0 * i as f64 + 4.0)) - 
            (1.0 / (8.0 * i as f64 + 5.0)) - (1.0 / (8.0 * i as f64 + 6.0)));
    }

    if digit > precision {
        format!("{:.*}", (precision+1) as usize, pi)
    } else {
        format!("{:.*}", (digit+1) as usize, pi)
    }
    
}

// Presents the user with the sub-menu for calculating the digits of pi

fn pi_menu() {
    clear_screen();
    
    let mut exit = false;
    
    while !exit {
        let mut valid = false;
        let mut numeric_input = 0;
        let mut digit = 5000;
        let mut input = String::new();
        
        while !valid {

            println!("Please enter the number of digits you wish to calculate pi to: ");
            println!{"\tNote: Maximum of 47 digits\n"}
            println!("\t- enter Q to quit calculating Pi. ");
    
            input.clear();
            io::stdin().read_line(&mut input);
    
            input.pop();
            input = input.to_uppercase();

            if input == "Q" {
                valid = true;
                exit = true; 
            }
            
            if !valid && input.len() > 0 {

                if check_if_number(&input) {
                    let test_num = input.parse::<u64>().unwrap();

                    valid = validate_input(test_num, 3);
    
                    if valid {
                        numeric_input = test_num;
                    }
                } else {
                    println!("\n\nInvalid input: Please try again");
                    println!("--------------------------------\n\n");
                    break;
                }
            }

            if input != "Q" && valid && input.len() !=0 {
                let mut inner_valid = false;
                let mut inner_input = String::new();
                while !inner_valid {

                    println!("\nIf you wish to see a particular digit, enter it. Otherwise press enter.\n");
                    inner_input.clear();
                    io::stdin().read_line(&mut inner_input);
            
                    inner_input.pop();
                    inner_input = inner_input.to_uppercase();


                    if inner_input.len() == 0 {
                        inner_valid = true;
                    } else {
                        inner_valid = check_if_number(&inner_input);
                        if inner_valid {
                            let digit_num = inner_input.parse::<u64>().unwrap();
        
                            inner_valid = validate_input(digit_num, 3);
            
                            if inner_valid && digit_num <= numeric_input {
                                digit = digit_num;
                            }
                        }
                    }


                } 

                let pi_output= digits_of_pi(digit, numeric_input);

                if digit > numeric_input {
                    println!("\n\nPi to {} digits is: {}\n\n", numeric_input, pi_output);
                } else {
                    let digit_char = pi_output.chars().nth((digit+2) as usize).unwrap();

                    println!("\n\nThe digit of pi you requested, {}, is: {}\n\n", digit, digit_char);
                }

            }
        }

    }
}

// Entry point for program and main menu

fn main() {
    let mut exec = true;            // tells the program to keep running
    let mut input = String::new();
    let mut first = true;
 
    clear_screen();

    println!("This program allows you to execute several recursive algorithms.\n");
 
    while exec {
        if !first {
            clear_screen();
        } else {
            first = false;
        }
        
        println!("Make your selection: \n");
        println!("\t\t-- 1) Factorial");
        println!("\t\t-- 2) Fibonacci Sequence");
        println!("\t\t-- 3) Digits of of pi");
        println!("\n\t- enter Q to exit program.\n");
        
        input.clear();
        io::stdin().read_line(&mut input);

        input.pop();
        input = input.to_uppercase();
        if input.len() > 0 {

            match input.chars().nth(0).unwrap() {
                '1' => {
                    factorial_menu();
                },
                '2' => {
                   fibonacci_menu();
                },
                '3' => {
                   pi_menu();
                },
                'Q' => {
                    exec = false; break;
                }
                _ => {
                    println!("\nInvalid selection.\n");
                }
            }
        } 
    }

    println!("\n\nThank you for using my extremely fun math program.\n\n")
}
