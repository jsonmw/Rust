/*

This program accepts as input a truth table and generates a function.
The goal was also to have it accept a sum of products function and
return the truth table, but I was having trouble learning Rust and
figuring out how to implement that level of string processing and 
complex logic in it.

*/

use std::io;

struct TruthTable {
    rows: Vec<String>,
    function: String
}

fn eval_prop() {
    // Evaluates an expression
    // if + then, if * then, if contains /
    // recursive
}

// Accepts user input to enter a truth table line by line and stores
// it in a Vector of Strings, which it also returns.

fn create_truth_table() -> Vec<String> {

    let mut input=String::new();
    let mut valid = false;  // if the given input is a legit row
    let mut count = 0;      // current row 
    let mut max_count = 99; // later assigned 2^n where n is # of variables
    let mut length = 0;     // the # of variables in the given row
    let mut table: Vec<String> = Vec::new();
    

    while count != max_count  {

        input.clear();

        // After the first iteration, displays current truth table in progress
    
        if count > 0 {

            print!("{}[2J", 27 as char); // clears console screen
            println!("CURRENT TRUTH TABLE");
            println!("-------------------");

            for s in &table {
                println!("\t{}",s)
            }

            println!("\t\t\tYou must enter {} more lines", (max_count-count));
            println!("\n\n")
        }

        println!("Please enter a truth table in binary form, pressing ENTER after each row");
        println!("\tUse up to 4 variables and include the output, ex: 01010 <enter> 01111 <enter>");

        io::stdin().read_line(&mut input);

        input.pop();  // remove '\n'
        valid = true;
        input = input.replace(" ", "").to_uppercase();

        // fixes the length for future iterations and sets the number of 
        // required rows (2^n) for the given number of variables.

        if count == 0 {
            length = input.len()-1;
            if length > 0 {
                max_count = (1 << length);
            }
        }


        if !table.contains(&input) && (length > 0 && length < 5) && (input.len()-1 == length)  {   
            for c in input.chars() {
                match c {
                    '0' | '1' => {
                        continue;
                    },
                    _ => {valid = false;}
                }
            }
            
        } else {
            valid = false;
        }

        if !valid {
            print!("{}[2J", 27 as char); // clears console screen
            println!("Invalid input\n");
        }

        // ouputs the current line to the truth table

        if valid {
            table.push(input.clone());
            table.sort();
            count+=1;
        }
    }

    derive_function(&table);
    table
}

// Accepts a reference to a Vector of strings holding a truth table, generates a corresponding
// function, and outputs it to the console. Returns the string holding the function.

fn derive_function(table: &Vec<String>) -> String {
    let mut output = String::new();

    for p in table {
        if p.ends_with('1') {          
            for v in p.chars().enumerate() {
                if v.1.eq(&'1') && v.0.ne(&(p.len()-1)) {
                    match v.0 {
                        0 => output.push_str("A"), 
                        1 => output.push_str("B"), 
                        2 => output.push_str("C"), 
                        3 => output.push_str("D"),
                        _ => output.push_str("") 
                    }
                } else if v.1.eq(&'0') && v.0.ne(&(p.len()-1)) {
                    match v.0 {
                        0 => output.push_str("/A"),
                        1 => output.push_str("/B"),
                        2 => output.push_str("/C"),
                        3 => output.push_str("/D"),
                        _ => output.push_str("")
                    }
                }
            } 
            output.push_str(" + ");
        }
    }

    print!("{}[2J", 27 as char); // clears console screen
    println!("TRUTH TABLE:\n");

    match table[0].len() {
        2 => {println!("A | F"); println!("-----");}
        3 => {println!("A B | F"); println!("-------");},
        4 => {println!("A B C | F"); println!("---------");},
        5 => {println!("A B C D | F"); println!("-----------");}
        _ => println!("")
    };
    
    for s in table {
        let length = s.len() -1;
        let mut current = 0;
        for c in s.chars() {
            if(current == length) {
                print!("| {} ",c);
            } else {
                print!("{} ",c);
            }
            current = current+1;
        }
        println!("");
    }

    println!("\nFUNCTION:\n\n{}\n", output.trim_end_matches(" + "));
    output
}

fn sum_of_products() {

    let mut input=String::new();
    let mut valid = false;

    while !valid {
        println!("Please enter a sum of products in the form (A*B) + /(C+D)");
        println!("\tUSE * for AND + for OR / for NOT");
        
        io::stdin().read_line(&mut input);
        input.pop();
        valid = true;

        if input.len() > 0 {
            input = input.replace(" ", "").to_uppercase();

            for c in input.trim().chars() {
                println!("Checking {}", c);
                print!("{}[2J", 27 as char); // clears console screen
                match c {
                    'A' | 'B' | 'C' | 'D' | '(' | ')' | '+' | '*' | '/' => {  
                        continue;
                    },
                    '\n' => {
                        break;
                    },
                    _ => {valid = false;}
                }
            }
        } else {
            valid = false;
        }

        if !valid {
            println!("Invalid input\n");
        }

    }
    evaluate_expression(&input);
}


fn evaluate_expression(expression: &String) {

    // parse til first +
    //
}

fn main() {
    print!("{}[2J", 27 as char); // clears console screen

    let mut input=String::new();
    let mut valid = false;

    println!("Welcome to LOGIC WORLD");
    println!("----------------------\n");

    while !valid {
        input.clear();
        println!("Choose which option you would like to do:\n");
        println!("\t(1) input a binary truth table\n\t(2) enter a sum of products function");
        println!("\t(3) exit\n");

        io::stdin().read_line(&mut input);
        input.pop();
        valid = true;
 
        if input.len() == 1 { 
            input = input.to_uppercase();
            for c in input.trim().chars() {
                print!("{}[2J", 27 as char); // clears console screen
                match c {
                    '1' => {
                        create_truth_table(); 
                    },
                    '2' => {
                        sum_of_products();
                    },
                    '3' => {
                        break;
                    }
                    _ => {valid = false;}
                }
            }   
        } else {
            valid = false;
        }
        if !valid {
            println!("Invalid input\n\n");
        }
    }
    
    println!("Thank you for using LOGIC WORLD");
    println!("-------------------------------\n");
    
}


                        // let truth = truth_table();
                        // let func = derive_function(&truth);
                        // let current = TruthTable {
                        //     rows: truth,
                        //     function: func
                        // };  