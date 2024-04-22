// Attempt at a Deduction Machine for Logical Predicates
// Jason Wild

    // TODO: 

    // Loop through the premises from the len()-1th, checking from 0 > len()-1
    // if contains ">", left side key in implies, right side value
    // after checking and handling ">" iterate through variables
    // If no ">" in line, then ^ means both variables are true, v adds no new info
    // if ">" check if right side truth is known, if not keep moving
    //      if known and true, single variable and ^ variable are true
    //      if known and false, single variable and ^ variable are false
    //      if not known and single variable or ^ with 2 known truths (or v with 1 known truth), right side is true
    //      if true, add to implies HashMap, add relevant variables or (expr) to known_truths
    // ^ Temporary variable expr=line.chars().collect() to create a vector of the characters of the given expression
    //   if "(", include all chars until the ")" and check against known_truths, the implies side is true or not true based on that expr
    
    // Two consultant data structures. a Vector of Structs to hold the variable and it's validity and a HashMap of implies

use std::io;
use std::collections::HashMap;

// Holds information about a particular premise

struct Truth {
    string: String,
    truth: i32,
    implies: String
}

// Check for modus_ponens

fn is_modus_ponens(expression: &String) -> bool {
    // check [1] for > operator
    // if found, store values
    // P > Q
    // P
    // Therefore Q
    false
}

// Check for modus_tollens 

fn is_modus_tollens(expression: &String) -> bool {
    // check [1] for > operator
    // if found, store values
    // P > Q
    // \P
    // Therefore \Q
    false
}

// Check for law of syllogism

fn is_law_of_syllogism(expression: &String) -> bool {

    false
}

// Validates the truth of a given premise by comparing it to known truths.

fn validate_premise(premise: &String) -> i32 {

    42

}

// Accepts a premise as a String and a vector of Truth structs and returns
// a boolean of whether it is know. Also adds truth to known truths if not
// already known.

fn contains_premise(premise: String, known_truths: &mut Vec<Truth>) -> bool {
    let mut contains = false;
    for variable in known_truths.into_iter() {
        if variable.string == premise {
            contains = true;
        }
    }

    if !contains {
        if premise != "A" && premise !="E" && premise !="R" {
            let newVar = Truth {
                string: premise.to_string(),
                truth: -1,  // implies "unknown", 0 = "false", 1 = "true"
                implies: "".to_string()
            };
            //println!("adding {} to known_truths", premise);
            known_truths.push(newVar);

        } else {
            contains = true;
        }
    }
    contains
}

fn main() {

    let mut input=String::new();
    let mut valid = false;  // if the given input is a legit row
    let mut count = 0;
    let mut premise_list: Vec<String> = Vec::new();
    let mut known_truths: Vec<Truth> = Vec::new();
    let mut implies: HashMap<String,String>= HashMap::new();
    let mut output: Vec<String>=Vec::new();
    
    while !valid  {

        input.clear();

        // After the first iteration, displays current premise in progress
        
        print!("{}[2J", 27 as char); // clears console screen
        if count > 0 {

            println!("Current premises");
            println!("-------------------");

            for s in &premise_list {
                println!("\t{}",s)
            }
            println!("");
        }

        println!("Please enter a series of premises line by line, separated by ENTER KEY");
        println!("Note: the capital A, E, R characters may not be used as variable names.\n");
        println!("\t> : implies");
        println!("\t~ : NOT");
        println!("\t* : AND");
        println!("\t+ : OR");
        println!("\tA : All elements");
        println!("\tE : Some element");
        println!("\tR : Therefore");
        println!("\nEnd with an R/Therefore statement or press ENTER with no input to validate.\n");

        io::stdin().read_line(&mut input);

        input.pop();  // remove '\n'
        valid = true;
        input = input.replace(" ", "");

        // Checks for repeat premises

        if !premise_list.contains(&input)  {   
            for c in input.chars() {
                if (c.is_alphabetic() || c.is_digit(10)) || c == '>' || c == '~' || c == '*' || c == '+' || c == '(' || c == ')' {
                    valid = true;
                } else {
                    valid = false;
                    break;
                }
            }
            
        } else {
            valid = false;
        }

        if !valid {
            print!("{}[2J", 27 as char); // clears console screen
            println!("Invalid input\n");
        }

        // ouputs the current line to the truth premise_list

        if valid && input.len() != 0 {
            premise_list.push(input.clone());
            valid = false;
        }
        count+=1;

        // Checks for enter press or R statement to end input

        if input.len() == 0 || input.chars().nth(0).unwrap() == 'R' {
            valid = true;
        }
    }

    println!("\nPremise list: {:?}", premise_list);

    // Iterates through premise_list, to populate known_truths

    for premise in premise_list {
        let mut output_promise = "";
        
        if premise.contains('>') {

            let implies_position = premise.find('>').unwrap();
            
            let left_side = &premise[..implies_position];
            let right_side = &premise[implies_position+1..];

            for c in left_side.chars() {
                if c.is_alphabetic() || c.is_digit(10) {
                    contains_premise(c.to_string(), &mut known_truths);
                }
            }
            for c in right_side.chars() {
                if c.is_alphabetic() || c.is_digit(10) {
                    
                    contains_premise(c.to_string(), &mut known_truths);
                }
            }
            
            implies.insert(left_side.to_string(),right_side.to_string());
        } else {
            for c in premise.chars() {
                if c.is_alphabetic() || c.is_digit(10) {
                    contains_premise(c.to_string(), &mut known_truths);
                }
            }
        }
    }


    
    println!("\nKnown variables:\n");
    for premise in &known_truths {
        print!("{}, ", premise.string);
    }
    println!();

    println!("\nKnown implications:\n");

    for (left, right) in &implies {
        println!("{} > {}", left, right);
    }
    
}
