// This program allows the user to input "tables" as comma separated values
// either via the keyboard or through files, and then allows for operations
// to be performed on them.

use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

// Clears the screen

fn clear_screen() {
    print!("{}[2J", 27 as char);
}


// Returns the table number input by the user after validating.

fn get_table_number(max: i32) -> i32 {
    let mut input = String::new();
    let mut valid_number = false;
    println!("\nEnter the table number you wish to operate on (Q to exit table entry): \n");
    input.clear();
    io::stdin().read_line(&mut input);
    input.pop();
    input = input.to_uppercase();
    while !valid_number {
        if input.chars().nth(0).unwrap().is_digit(10) {
            let table_number = input.parse::<i32>().unwrap();
            return table_number as i32;
        } else if input.chars().nth(0).unwrap() == 'Q' {
            valid_number = true;
        } else {
            println!("\nInvalid table number, please enter a number between 1 and {}:\n", (max+1));
            input.clear();
            io::stdin().read_line(&mut input);
        }
        
    }
    return -1;
}
 
// Validates the input for the table entry information. Returns
// integer corresponding ot user choice.

fn input_decision() -> i32 {
    let mut input = String::new();
    let mut valid = false;

    println!("Select how you wish to enter table information:\n");
    println!("--------------------------------------------------------\n");
    while !valid {

        println!("\t1 - Keyboard Prompt");
        println!("\t2 - From file");
        println!("\nPress Q to exit table entry.\n");
        
        input.clear();
        io::stdin().read_line(&mut input);

        input.pop();
        input = input.to_uppercase();
        if input.len() > 0 {

            match input.chars().nth(0).unwrap() {
                '1' => {
                    return 1;
                },
                '2' => {
                   return 2;
                },
                'Q' => {
                    return -1;
                }
                _ => {
                    println!("\nInvalid selection.\n");
                }
            }
        } 
    }

    -1
}

// Accepts a table as input from the keyboard in the form
// element, element, element, element. Returns the values
// as a HashSet.

fn input_words() -> HashSet<String> {
    let mut new_table = HashSet::new();
    println!("\nEnter elements, separated by commas:\n");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    for word in input.split(',') {
        new_table.insert(word.trim().to_string());
    }

    new_table
}

// Accepts a table as input from a text file in the form
// element, element, element, element. Returns the values
// as a HashSet.

fn input_words_from_file() -> HashSet<String> {
    let mut filename = String::new();
    let mut valid_file = false;
    let mut input = String::new();
    
    while !valid_file {
        input.clear();
        println!("\nPlease enter the file name containing your table (must be in folder with program or provide absolute path):\n");
        io::stdin().read_line(&mut input);

        input.pop();
        input = input.to_lowercase().trim().to_string();

        if Path::new(&input).is_file() {
            valid_file = true;
            filename = input.clone();
        } else {
            println!("\nInvalid file. Please try again.\n");
        }

    }

    let file = match File::open(filename) {
        Ok(file) => file,
        Err(error) => {
            println!("Error opening file '{}': {}", input, error);
            return HashSet::new();
        }
    };

    let reader = BufReader::new(file);
    let mut new_table = HashSet::new();

    for line in reader.lines() {
        let line = line.unwrap();
        for word in line.split(',') {
            new_table.insert(word.trim().to_string());
        }
    }

    new_table
}

// This excepts a Vector slice of HashSets representing tables and
// a boolean indicating if the table name is to be printed with the
// table elements and prints them to the console.

fn view_table(tables: &[HashSet<String>], table_name: bool) {
    let mut current_set = 1;
    let mut current_el = 0;
    println!();
    for table in tables {
        current_el = 0;
        if table_name {
            print!("Table {}: ", current_set);
        }
        print!("{{");
        for element in table {
            if current_el < table.len()-1 {
                print!("{}, ", element.to_string());
            } else {
                print!("{}", element.to_string());
            }
            current_el+=1;
        }
        println!("}}");
        current_set +=1;
    }
}

// This function accepts a Vector slice of HashSets representing tables and
// allows the user to perform an INNER JOIN on them. It simulates an SQL 
// inner join by unioning the selected sets together. The max parameter 
// is a limit on the number of joins that can be done based on how many sets 
// the user entered.

fn inner_join(tables: &[HashSet<String>], max: i32) {
    let mut input = String::new();
    let mut number_of_joins = 0;
    let mut get_variables = true;
    let mut variables: Vec<HashSet<String>> = vec![];
    let mut table_numbers: Vec<usize> = vec![];
    let mut result_set = HashSet::<String>::new();
    
    println!("Select the tables you wish to include in the inner join: ");
    let first_table = get_table_number(max) as usize;
    variables.push(tables[first_table-1 as usize].clone());
    table_numbers.push(first_table);

    while get_variables {
        if number_of_joins < max {
            println!("\nCurrent number of joins : {} / Max {}\n", number_of_joins, max);

            let current_table = get_table_number(max);
            if current_table > 0 {
                variables.push(tables[(current_table-1) as usize].clone());
                table_numbers.push(current_table as usize);
                number_of_joins +=1;
            } else {
                break;
            }
        } else {
            get_variables =false;
            break;
        }
    } 
    
    
    for i in 0.. number_of_joins {
        result_set = variables[i as usize].intersection((&variables[(i+1) as usize])).cloned().collect();

    }

    clear_screen();
    print!("\nResult set for INNER JOIN {}", table_numbers[0]);
    for i in 1.. table_numbers.len() {
        print!(" ∩ {}", table_numbers[i as usize]);
    }
    println!("");

    println!("----------------------------------------");
    view_table(&[result_set], false);
    println!("\n----------------------------------------\n");

    println!("Press enter to continue.");
    io::stdin().read_line(&mut input);

}

// This function accepts a Vector slice of HashSets representing tables and
// allows the user to perform an OUTER JOIN on them. It simulates an SQL 
// outer join by unioning the selected sets together. The max parameter 
// is a limit on the number of joins that can be done based on how many sets 
// the user entered.

fn outer_join(tables: &[HashSet<String>]) {
    let max = 1;
    let mut input = String::new();
    let mut number_of_joins = 0;
    let mut get_variables = true;
    let mut variables: Vec<HashSet<String>> = vec![];
    let mut table_numbers: Vec<usize> = vec![];
    let mut left_side = HashSet::<String>::new();
    let mut right_side = HashSet::<String>::new(); 
    let mut result_set = HashSet::<String>::new();
    
    println!("Select the tables you wish to include in the outer join: ");
    let first_table = get_table_number(max) as usize;
    variables.push(tables[first_table-1 as usize].clone());
    table_numbers.push(first_table);

    while get_variables {
        if number_of_joins < max {
            println!("\nCurrent number of joins : {} / Max {}\n", number_of_joins, max);

            let current_table = get_table_number(max);
            if current_table > 0 {
                variables.push(tables[(current_table-1) as usize].clone());
                table_numbers.push(current_table as usize);
                number_of_joins +=1;
            } else {
                break;
            }
        } else {
            get_variables =false;
            break;
        }
    } 
    
    
    for i in 0.. number_of_joins {
        
        left_side = variables[i as usize].difference((&variables[(i+1) as usize])).cloned().collect();
        right_side = variables[(i+1) as usize].difference((&variables[i as usize])).cloned().collect();
        
        result_set = left_side.union(&right_side).cloned().collect();
    }

    clear_screen();
    print!("\nResult set for OUTER JOIN {}", table_numbers[0]);
    for i in 1.. table_numbers.len() {
        print!(" ∪ {}", table_numbers[i as usize]);
    }
    println!("");
    println!("----------------------------------------");
    view_table(&[result_set], false);
    println!("\n----------------------------------------\n");

    println!("Press enter to continue.");
    io::stdin().read_line(&mut input);

}

// This function accepts a Vector slice of HashSets representing tables and
// allows the user to perform a FULL OUTER JOIN on them. It simulates an SQL 
// full outer join by unioning the selected sets together. The max parameter 
// is a limit on the number of joins that can be done based on how many sets 
// the user entered.

fn full_outer_join(tables: &[HashSet<String>], max: i32) {
    let mut input = String::new();
    let mut number_of_joins = 0;
    let mut get_variables = true;
    let mut variables: Vec<HashSet<String>> = vec![];
    let mut table_numbers: Vec<usize> = vec![];
    let mut result_set = HashSet::<String>::new();
    
    println!("Select the tables you wish to include in the FULL OUTER JOIN: ");
    let first_table = get_table_number(max) as usize;
    variables.push(tables[first_table-1 as usize].clone());
    table_numbers.push(first_table);

    while get_variables {
        if number_of_joins < max {
            println!("\nCurrent number of joins : {} / Max {} \n", number_of_joins, max);

            let current_table = get_table_number(max);
            if current_table > 0 {
                variables.push(tables[(current_table-1) as usize].clone());
                table_numbers.push(current_table as usize);
                number_of_joins +=1;
            } else {
                break;
            }
        } else {
            get_variables =false;
            break;
        }
    } 
    
    for i in 0.. number_of_joins {
        let mut temp_result_set = HashSet::<String>::new();
        temp_result_set = variables[i as usize].union((&variables[(i+1) as usize])).cloned().collect();

        result_set.extend(temp_result_set);
    }

    clear_screen();
    print!("\nResult set for FULL OUTER JOIN {}", table_numbers[0]);
    for i in 1.. table_numbers.len() {
        print!(" ∪ {}", table_numbers[i as usize]);
    }
    println!("");

    println!("----------------------------------------");
    view_table(&[result_set], false);
    println!("\n----------------------------------------\n");

    println!("Press enter to continue.");
    io::stdin().read_line(&mut input);
}

// This function accepts a vector slice of HashSets representing tables
// and allows the user to perform a left join on them. The way I simulate 
// left join is by differencing the left side to the right, then unioning 
// that with the intersection of left and right to fill in the middle. This
// only allows operations between two sets.

fn left_join(tables: &[HashSet<String>]) {
    let max = 1;
    let mut input = String::new();
    let mut number_of_joins = 0;
    let mut get_variables = true;
    let mut variables: Vec<HashSet<String>> = vec![];
    let mut table_numbers: Vec<usize> = vec![];
    let mut left_side = HashSet::<String>::new();
    let mut middle_stuff = HashSet::<String>::new();
    let mut result_set = HashSet::<String>::new();
    
    println!("Select the tables you wish to include in the LEFT JOIN: ");
    let first_table = get_table_number(max) as usize;
    variables.push(tables[first_table-1 as usize].clone());
    table_numbers.push(first_table);
    
    while get_variables {
        if number_of_joins < max {
            println!("\nCurrent number of joins : {} / Max {}\n", number_of_joins, max);

            let current_table = get_table_number(max);
            if current_table > 0 {
                variables.push(tables[(current_table-1) as usize].clone());
                table_numbers.push(current_table as usize);
                number_of_joins +=1;
            } else {
                break;
            }
        } else {
            get_variables =false;
            break;
        }
    } 

    for i in 0.. number_of_joins {
        // Overly complicated, I could have just returned the first operand.. did as thought experiment
        let mut temp_result_set = HashSet::<String>::new();

        left_side = variables[i as usize].difference((&variables[(i+1) as usize])).cloned().collect();
        middle_stuff = variables[(i+1) as usize].intersection((&variables[i as usize])).cloned().collect();
        temp_result_set = left_side.union(&middle_stuff).cloned().collect();
        result_set = result_set.union(&temp_result_set).cloned().collect();
    }

    clear_screen();
    print!("\nResult set for LEFT JOIN {}", table_numbers[0]);
    for i in 1.. table_numbers.len() {
        print!(" ∪ {}", table_numbers[i as usize]);
    }
    println!("");

    println!("----------------------------------------");
    view_table(&[result_set], false);
    println!("\n----------------------------------------\n");

    println!("Press enter to continue.");
    io::stdin().read_line(&mut input);
}


// This function accepts a vector slice of HashSets representing tables
// and allows the user to perform a right join on them. The way I simulate 
// left join is by differencing the right side to the left, then unioning 
// that with the intersection of left and right to fill in the middle. This 
// only allows operations between two sets.

fn right_join(tables: &[HashSet<String>]) {
    let max = 1;
    let mut input = String::new();
    let mut number_of_joins = 0;
    let mut get_variables = true;
    let mut variables: Vec<HashSet<String>> = vec![];
    let mut table_numbers: Vec<usize> = vec![];
    let mut right_side = HashSet::<String>::new();
    let mut middle_stuff = HashSet::<String>::new();
    let mut result_set = HashSet::<String>::new();
    
    println!("Select the tables you wish to include in the LEFT JOIN: ");
    let first_table = get_table_number(max) as usize;
    variables.push(tables[first_table-1 as usize].clone());
    table_numbers.push(first_table);

    while get_variables {
        if number_of_joins < max {
            println!("\nCurrent number of joins : {} / Max {}\n", number_of_joins, max);

            let current_table = get_table_number(max);
            if current_table > 0 {
                variables.push(tables[(current_table-1) as usize].clone());
                table_numbers.push(current_table as usize);
                number_of_joins +=1;
            } else {
                break;
            }
        } else {
            get_variables =false;
            break;
        }
    } 
    
    for i in (0.. number_of_joins).rev() {
        // Overly complicated, I could have just returned the first operand.. did as thought experiment
        let mut temp_result_set = HashSet::<String>::new();

        right_side = variables[(i+1) as usize].difference((&variables[i as usize])).cloned().collect();
        middle_stuff = variables[(i+1) as usize].intersection((&variables[i as usize])).cloned().collect();
        temp_result_set = right_side.union(&middle_stuff).cloned().collect();
        result_set = result_set.union(&temp_result_set).cloned().collect();
    }

    clear_screen();
    print!("\nResult set for RIGHT JOIN {}", table_numbers[0]);
    for i in 1.. table_numbers.len() {
        print!(" ∪ {}", table_numbers[i as usize]);
    }
    println!("");

    println!("----------------------------------------");
    view_table(&[result_set], false);
    println!("\n----------------------------------------\n");

    println!("Press enter to continue.");
    io::stdin().read_line(&mut input);
}

// This function accepts a vector slice of HashSets representing tables
// and allows the user to perform a difference on them. This represents 
// a LEFT or RIGHT join ON NULL in SQL, depending on which table is entered first.

fn set_difference(tables: &[HashSet<String>], max: i32) {
    let mut input = String::new();
    let mut number_of_joins = 0;
    let mut get_variables = true;
    let mut variables: Vec<HashSet<String>> = vec![];
    let mut table_numbers: Vec<usize> = vec![];
    let mut result_set = HashSet::<String>::new();
    
    println!("Select the tables you wish to include in the DIFFERENCE: ");
    let first_table = get_table_number(max) as usize;
    variables.push(tables[first_table-1 as usize].clone());
    table_numbers.push(first_table);

    while get_variables {
        if number_of_joins < max {
            println!("\nCurrent number of differences : {} / Max {}\n", number_of_joins, max);

            let current_table = get_table_number(max);
            if current_table > 0 {
                variables.push(tables[(current_table-1) as usize].clone());
                table_numbers.push(current_table as usize);
                number_of_joins +=1;
            } else {
                break;
            }
        } else {
            get_variables =false;
            break;
        }
    } 
    
    for i in 0.. number_of_joins {
        result_set = variables[i as usize].difference((&variables[(i+1) as usize])).cloned().collect();
    }

    clear_screen();
    print!("\nResult set for DIFFERENCE {}", table_numbers[0]);
    for i in 1.. table_numbers.len() {
        print!(" \\ {}", table_numbers[i as usize]);
    }
    println!("");

    println!("----------------------------------------");
    view_table(&[result_set], false);
    println!("\n----------------------------------------\n");

    println!("Press enter to continue.");
    io::stdin().read_line(&mut input);
}

fn main() {
    let MAX_TABLES = 5;
    let mut number_of_tables = 0;
    let mut tables = Vec::new();
    let mut input = String::new();
    let mut first = true;
    let mut exec = true;

    clear_screen();

    println!("This program allows you to perform operations on tables.\n");

    let mut decision = 0;
    
    while number_of_tables < MAX_TABLES {
        if !first {
            clear_screen();
            if number_of_tables < 2 {
                println!("You must add at least two table to perform operations.\n");
            } else {
                println!("\nWould you like to add another table? Current tables: {} ({} maximum)\n", number_of_tables, MAX_TABLES);
            }
            print!("-----------------------------------------------");
            view_table(&tables, true);
            println!("-----------------------------------------------\n");
        } else {
            first = false;
        }

        decision = input_decision();
        match decision {
            1 => {
                tables.push(input_words());
                number_of_tables+=1;
            },
            2 => {
                tables.push(input_words_from_file());
                number_of_tables+=1;
            },
            -1 => {
                if number_of_tables >= 2 {
                    break;
                }
            },
            _ => {
                println!("\nInvalid entry\n")
            }
        }
    } 

    while exec {     
        clear_screen();

        print!("-----------------------------------------------");
        view_table(&tables, true);
        println!("-----------------------------------------------\n");

        println!("\nWhich operation would you like to perform?\n");
        println!("\t 1 - INNER JOIN");
        println!("\t 2 - OUTER JOIN");
        println!("\t 3 - FULL OUTER JOIN");
        println!("\t 4 - LEFT JOIN");
        println!("\t 5 - RIGHT JOIN");
        println!("\t 6 - DIFFERENCE\n");
        println!("Q - Exit.\n");

        input.clear();
        io::stdin().read_line(&mut input);

        input.pop();
        input = input.to_uppercase();
        
        if input.len() > 0 {
            match input.chars().nth(0).unwrap() {
                '1' => {
                    inner_join(&tables, number_of_tables-1);
                },
                '2' => {
                    outer_join(&tables);
                },
                '3' => {
                    full_outer_join(&tables, number_of_tables-1);
                },
                '4' => {
                    left_join(&tables);
                },
                '5' => {
                    right_join(&tables);
                },
                '6' => {
                    set_difference(&tables, number_of_tables-1);
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

    println!("\n\nThank you for using my bargain bin SQL.\n\n")

}