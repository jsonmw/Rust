// This program allows the user to input a relation and determine
// whether or not it has certain relational properties. 

use std::io;

// Clears the console of output.

fn clear_screen() {
    print!("{}[2J", 27 as char);
}

// This checks to see if the given relation, as a Vector of Strings, is reflexive
// under the given Set, as a String, and returns a boolean result. It also accepts 
// a boolean that will output text if needed.

fn check_reflexive(L: &Vec<String>, S: &String, display_text: bool) -> bool {
    
    if display_text {
        clear_screen();
        println!("Checking if relation is REFLEXIVE...\n");
    }
    
    let set_elements: Vec<char> = S.chars().filter(|&bracket| bracket != '{' && bracket != '}').collect();

    for element in &set_elements {
        if element != &',' && element != &' ' {
            let relation = format!("({},{})", element, element);

            if !L.contains(&relation.to_string()) {
                return false;
            }
        }
    }
    true
}

// This checks to see if the given relation, as a Vector of Strings, is symmetric
// and returns a boolean result. It also accepts a boolean that will output text
// if needed.

fn check_symmetric(L: &Vec<String>, S: &String, display_text: bool) -> bool {
    
    
    if display_text {
        clear_screen();
        println!("Checking if relation is SYMMETRIC...\n");
    }
    
    // Iterates through the relation list and checks to see if each pair's
    // reversed form exists in the list

    for element in L {
    
        let split_pair: Vec<&str> = element.trim_matches(|paren| paren == '(' || paren == ')').split(',').collect();
        let symmetric_pair = format!("({},{})", split_pair[1], split_pair[0]); // creates a flipped pair.

        if !L.contains(&symmetric_pair.to_string()) {
            return false;
        }
    }
    true
}

// This checks to see if the given relation, as a Vector of Strings, is transitive
// and returns a boolean result. It also accepts a boolean that will output text
// if needed.

fn check_transitive(L: &Vec<String>, display_text: bool) -> bool {
    
    if display_text {
        clear_screen();
        println!("Checking if relation is TRANSITIVE...\n");
    }
    
    // This nested loop iterates through each pair in the list, and checks it against
    // the others to determine transitivity.

    for i in 0..L.len() {
        
        // this stores the first pair to be checked
        let first_pair: Vec<&str> = L[i].trim_matches(|paren| paren == '(' || paren == ')').split(',').collect();
        
        //iterates through the rest of the relation checking each pair as it goes
        for j in (i+1)..L.len() {
            let second_pair: Vec<&str> = L[j].trim_matches(|paren| paren == '(' || paren == ')').split(',').collect();
            
            // These two if's check for transitive chain by flipping the variables and seeing if the
            // new pair is contained in the list
            
            if first_pair[1] == second_pair[0] {
                let checked_pair = format!("({},{})", first_pair[0], second_pair[1]);

                if !L.contains(&checked_pair.to_string()) {
                    return false;
                }
                
            }
            
            if second_pair[1] == first_pair[0] {
                
                let checked_pair = format!("({},{})", second_pair[0], first_pair[1]);

                if !L.contains(&checked_pair.to_string()) {
                    return false;
                }
            }
        }
    } 
    true
}

// This checks to see if the given relation, as a Vector of Strings, is anti 
// symmetric and returns a boolean result. It also accepts a boolean that will 
// output text if needed.

fn check_antisymmetric(L: &Vec<String>, S: &String, display_text: bool) -> bool {
    
    if display_text {
        clear_screen();
        println!("Checking if relation is ANTISYMMETRIC...\n");
    }

    for element in L {
        let split_pair: Vec<&str> = element.trim_matches(|paren| paren == '(' || paren == ')').split(',').collect();
        let (x, y) = (split_pair[0], split_pair[1]);
        if x == y {
            continue; // skip pairs with the same elements
        }

        let symmetric_pair = format!("({},{})", y, x);
        if L.contains(&symmetric_pair) {
            return false;
        }
    }
    true
}

// This checks to see if the given relation, as a Vector of Strings, is anti 
// equivalent and returns a boolean result. It also accepts a boolean that will 
// output text if needed.

fn check_equivalence(L: &Vec<String>, S: &String, display_text: bool) -> bool {
    
    if display_text {
        clear_screen();
        println!("Checking if relation is EQUIVALENT...\n");
    }

    if check_reflexive(&L, &S, false) && check_symmetric(&L, &S, false) && check_transitive(&L, false) {
        return true;
    }

    false
}

// Prompts the user to enter a list, breaks it apart, and returns the elements
// of the given relation as a Vector of Strings.

fn get_list() -> Vec<String> {
    let mut input = String::new();
    let mut valid = false;
    let mut list: Vec<String> = Vec::new();
    
    println!("\nEnter a list of pairs of a relation in the form \"(a,b), (1,2), (+,*), ...\": ");
    println!("----------------------\n");
    io::stdin().read_line(&mut input);

    let mut new_pair = String::new();
    let mut open = false;

    for c in input.chars() {
        match c {
            '(' => {
                if !open {
                    new_pair.clear();
                    new_pair.push('(');
                }
                open = true;
            }
            ')' => {
                open = false;
                new_pair.push(')');
                list.push(new_pair.trim().to_string());
            }
            _ => {
                if open && c != ' '{
                    new_pair.push(c);
                }
            }
        }
    }

    list
}

// Prompts the user to input a Set as a String, and returns it.

fn get_set() -> String {
    let mut set = String::new();
    let mut input = String::new();

    println!("\nEnter a set of elements in the form \"{{a,b,c,d,...}}\": ");
    println!("----------------------\n");
    io::stdin().read_line(&mut input);
    
    input.trim().to_string()
}

fn print_set_and_relation(L: &Vec<String>, S: &String) {
    print!("\nCurrent relation:  ");
    for relation in L {
        print!("{} ", relation);
    }
    println!();
    println!("Current set:       {}\n", S);
}

// Main method, contains menu logic and overall program flow.

fn main() {
    let mut input = String::new();
    let mut first = true;
    let mut exec = true;

    clear_screen();
    
    print!("\n");
    println!("                                 WELCOME ");
    println!("\n");
    println!("This program allows you to determine properties about particular relations.\n");

    let mut L = get_list();
    let mut S = get_set(); 
    
    while exec { 
        if !first {
        } else {
            clear_screen();
            first = false;
        }
        
        print_set_and_relation(&L, &S);
        print!("--------------------------------------------------\n");
        println!("What type of relation would you like to check for? ");
        println!("--------------------------------------------------\n");

        println!("\t 1 - Reflexive");
        println!("\t 2 - Symmetric");
        println!("\t 3 - Transitive");
        println!("\t 4 - Antisymetric");
        println!("\t 5 - Equivalence");
        println!("\t 6 - Enter New Relation\n");
        println!("Q - Quit.\n");

        input.clear();
        io::stdin().read_line(&mut input);

        input.pop();
        input = input.to_uppercase();
        
        if input.len() > 0 {
            match input.chars().nth(0).unwrap() {
                '1' => {
                    let reflexive = check_reflexive(&L,&S,true);
                    if reflexive {
                        println!("Quite reflexive, if I do say so.");
                    } else {
                        println!("Unfortunately, not reflexive.");
                    }
                },
                '2' => {
                    let symmetric = check_symmetric(&L, &S, true);
                    if symmetric {
                        println!("Quite symmetric, if I do say so.");
                    } else {
                        println!("Unfortunately, not symmetric.");
                    }
                },
                '3' => {

                    let transitive = check_transitive(&L, true);
                    if transitive {
                        println!("Quite transitive, if I do say so.");
                    } else {
                        println!("Unfortunately, not transitive.");
                    }
                },
                '4' => {
                    let antisymmetric = check_antisymmetric(&L, &S, true);
                    if antisymmetric {
                        println!("Quite antisymmetric, if I do say so.");
                    } else {
                        println!("Unfortunately, not antisymmetric.");
                    }
                },
                '5' => {
                    let equivalent = check_equivalence(&L, &S, true);
                    if equivalent {
                        println!("Quite equivalent, if I do say so.");
                    } else {
                        println!("Unfortunately, not equivalent.");
                    }
                },
                '6' => {
                    clear_screen();
                    L = get_list();
                    S = get_set(); 
                    clear_screen();
                },
                'Q' => {
                    exec = false;
                },
                _ => {
                    println!("\nInvalid input, please try again.\n");
                }
            }
        } else {
            println!("\nInvalid input, please try again.\n");
        }
    }

    clear_screen();
    println!("\n\nWhat fun we had.\n\n")
}