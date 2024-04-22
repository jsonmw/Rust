// This program allows a user to parse iptables firewall rules
// to determine if they are valid.

#[macro_use]
extern crate peg;
use std::io;
use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};

// Defines the parser that validates Firewall Rules

peg::parser! {
    grammar iptables_parser() for str {

        // rules for parsing

        // rule -> "ip tables " + target + chain(s) + jump target

        rule whitespace() -> ()
            = " " / "\t" / "\n" / "\r"
        
        // Non-terminating symbols 

        rule chain() -> () 
            = parameter_options() (parameter_options() (parameter_options() (parameter_options())*)*)*
        
        rule parameter_options() -> () 
            = source_dest_chain() / interface_chain() / protocol_chain() / m_chain() / dport_chain() / state_chain() / icmp_chain()

        rule interface_chain() -> ()
            = flags() + whitespace() + interface() + whitespace()

        rule protocol_chain() -> ()
            = flags() + whitespace() + protocol() + whitespace()

        rule source_dest_chain() -> ()
            = flags() + whitespace() + ipv4() + whitespace()

        rule m_chain() -> ()
            = flags() + whitespace() + "state" + whitespace()
        
        rule dport_chain() -> ()
            = flags() + whitespace() + ['0'..='9'] + whitespace()
        
        rule icmp_chain() -> ()
            = flags() + whitespace() + "echo-request" + whitespace()
        
        rule state_chain() -> ()
            = flags() + whitespace() + "NEW" + whitespace()

        // Terminating symbols
       
        rule network_prefix() -> ()
            = "/" + ['0'..='9'] + ['0'..='9']*
        rule ipv4() -> ()
         = ['0'..='9'] + "." + ['0'..='9'] + "." + ['0'..='9'] + "." + ['0'..='9'] + network_prefix()* 
        
        rule jump_target() -> ()
            = "-j ACCEPT" / "-j DROP"

        rule interface() -> ()
            = "eth1" / "eth" + ['0'..='9']*
        
        rule protocol() -> ()
            = "tcp" / "udp" / "icmp" 
        
        rule target() -> () 
            = "-A INPUT" / "-A OUTPUT" / "-A DROP" / "-D INPUT" / "-I FORWARD"
        
        rule flags() -> ()
            = "-s" / "-d" / "-i" / "-p" / "-m"  / "--dport" / "-sport" / "--sport" / "--icmp-type" / "--state"

        // function for parse

        pub rule parse_rule(input: &str) -> ()
            = "iptables " + target() + whitespace() + chain() + jump_target()
    }
}

// Calls the parser defined and loops through the given rule set

fn parse_rules(rule_set: &Vec<String>) {
    clear_screen();
    for rule in rule_set {

        match iptables_parser::parse_rule(&rule, &rule) {
            Ok(_) => println!("VALID:  {}", rule),
            Err(err) => println!("INVALID:  {}", rule),
        }
    }

    println!("\nPress any key to continue\n");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
}

// Clears the console screen of input

fn clear_screen() {
    print!("{}[2J", 27 as char);
}

// Prints the given working rule set to the console

fn print_current_rule_set(rule_set: &Vec<String>) {
    if rule_set.len() > 0 {
        println!("\nCurrent Rule Set: \n");
        for rule in rule_set {
            println!("{}", rule);
        }
        println!();
    }   
}

// Accepts user input for rule set and returns a Vector of Strings
// containing the rule set

fn get_rules_from_console() -> Vec<String> {
    let mut rule_set = Vec::<String>::new();
    let mut get_rules = true;
    let mut input= String::new();
    
    while get_rules {
        clear_screen();
        input.clear();
        print_current_rule_set(&rule_set);

        println!("Enter rules one line at a time. Enter 'Q' to finish.\n");
        io::stdin().read_line(&mut input);
        if input.to_uppercase().chars().nth(0).unwrap() == 'Q' {
            get_rules = false;
        } else {
            let mut new_rule = input.clone();
            new_rule.pop(); // get rid of the '\n'
            rule_set.push(new_rule); 
        } 
    }
    rule_set
}

// Accepts a user input file to populate the rule set and returns a Vector of 
// Strings containing the rule set

fn get_rules_from_file() -> Vec<String> {
    let mut rule_set = Vec::<String>::new();
    let mut valid_file = false;

    while !valid_file {
        let mut input = String::new();
        println!("\nPlease enter the name of the text file containing your rules.\n");
        io::stdin().read_line(&mut input).expect("Error reading input");

        let path = Path::new(input.trim());
        if !path.is_file() {
            println!("Error: Input path is not a file.");
            continue;
        } else {
            valid_file = true;
        }

        let file = File::open(path).expect("Error opening file");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            rule_set.push(line.expect("Error reading line"));
        }
    }

    clear_screen();
    print_current_rule_set(&rule_set);

    println!("Press any key to continue\n");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    rule_set
}

// Handles the menu logic

fn main() {
    let mut rules_list: Vec<String>;
    let mut exec = true;

    while exec {
        let mut input = String::new();
        clear_screen();

        println!("Welcome to Firewall Rule Validator.\n");
        println!("How would you like to enter your fire wall rules?");
        println!("\n\t1-Console prompt");
        println!("\t2-File");
        println!("\tQ-I actually hate firewall rules and want to quit.\n");
        
        io::stdin().read_line(&mut input);

        let c = input.to_uppercase().chars().nth(0).unwrap();

        match c {
            '1' => {
                rules_list = get_rules_from_console();
                parse_rules(&rules_list);
            },
            '2' => {
                rules_list = get_rules_from_file();
                parse_rules(&rules_list);
            },
            'Q' => {
                exec = false;
            },
            _ => {}
        }
    }

    clear_screen();
    println!("Thanks for using Firewall Rule Validator\n");

}