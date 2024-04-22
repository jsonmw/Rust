// I created a game where user can simulate performing exercise and
// gaining points while managing a stamina resource and racing against
// a given amount of turns. It uses enums to hold states, a struct
// to hold player data for the current game, and methods to handle
// changing states and for displaying game information to the player.

use std::io;

// The overall state of the player

#[derive(Debug)] // so I could use {:?}  to print 
enum PlayerState {
    Standing,
    Walking,
    Running,
    Jumping,
    Sitting,
    Laying,
    Falling,
    Dead,
}

// The state in which the player is facing

#[derive(Debug)] // so I could use {:?}  to print 
enum Direction {
    Left,
    Right,
}

// Holds data about the current player

struct Player {
    state: PlayerState,
    stamina: i32,
    score: i32,
    high_score: i32,
    direction: Direction,
}

// Implements methods for the player struct

impl Player {

    // Constructor method, accepts the current high score

    fn new(current_high_score: i32) -> Player {
        Player {
            state: PlayerState::Standing,
            stamina: 105,
            score: 0,
            high_score: current_high_score,
            direction: Direction::Right,
        }
    }

    // Handles state transitions based on user input (and, if relevant depending 
    // on the current state, the current direction)

    fn handle_input(&mut self) {

        let mut input = String::new();
        io::stdin().read_line(&mut input);
        let c = input.to_lowercase().chars().next().unwrap();

        match c {
            'k' => {
                match self.state {
                    PlayerState::Standing => {
                         self.direction = Direction::Right;
                         self.state = PlayerState::Walking;
                    },                
                    PlayerState::Walking => {
                        match self.direction {
                            Direction::Right => self.state = PlayerState::Running,
                            Direction::Left => self.state = PlayerState:: Standing,
                        }       
                    },
                    PlayerState::Running => {
                        match self.direction {
                            Direction::Right => self.state = PlayerState::Running,
                            Direction::Left => self.state = PlayerState::Walking,
                        }
                    },
                    _ => {} 
                }
            },
            'h' => {
                match self.state {
                    PlayerState::Standing => {
                         self.direction = Direction::Left;
                         self.state = PlayerState::Walking;
                    },                
                    PlayerState::Walking => {
                        match self.direction {
                            Direction::Left => self.state = PlayerState::Running,
                            Direction::Right => self.state = PlayerState:: Standing,
                        }
                    },
                    PlayerState::Running => {
                        match self.direction {
                            Direction::Left => self.state = PlayerState::Running,
                            Direction::Right => self.state = PlayerState::Walking,
                        }
                    },
                    _=> {} 
                }
            },
            'j' => {
                match self.state {
                    PlayerState::Standing => self.state = PlayerState::Jumping,
                    PlayerState::Sitting => self.state = PlayerState::Standing,
                    PlayerState::Falling => self.state = PlayerState::Laying,
                    PlayerState::Laying => self.state = PlayerState::Sitting,
                    _ => {} 
                }
            },
            'l' => {
                match self.state {
                    PlayerState::Jumping => self.state = PlayerState::Standing,
                    PlayerState::Running => self.state = PlayerState::Falling,
                    PlayerState::Walking => self.state = PlayerState::Falling,
                    PlayerState::Standing => self.state = PlayerState::Sitting,
                    PlayerState::Sitting => self.state = PlayerState::Laying,
                    PlayerState::Falling => self.state = PlayerState::Dead,
                    PlayerState::Laying => self.state = PlayerState::Dead,
                    _ => {} 
                }
            },
            _ => {
                match self.state {
                    PlayerState::Running => self.state = PlayerState::Falling,
                    PlayerState::Falling => self.state = PlayerState::Dead,
                    _=> {}
                }

            } 
        }
    }

    // Updates the player's score and stamina fields

    fn calculate_score(&mut self) {
        match self.state {
            PlayerState::Standing => {self.stamina -= 5;},
            PlayerState::Walking => {self.stamina -= 10; self.score +=10;},
            PlayerState::Running => {self.stamina -= 20; self.score +=20;},
            PlayerState::Jumping => {self.stamina -= 50; self.score +=60;},
            PlayerState::Sitting => {self.stamina += 10; self.score -=5;},
            PlayerState::Laying => {self.stamina += 30; self.score -=5;},
            PlayerState::Falling => {self.score -=20;},
            _=> {},
        }
    }
    
    // Prints options to the user based on the current state and direction

    fn print_options(&mut self) {
        match self.state {
            PlayerState::Standing =>println!("\tCurrently Standing:\n\n\th: Walk Left\n\tk: Walk Right\n\tj: Jump\n\tl: Have a seat\n\n\tENTER: Keep standing\n"),
            PlayerState::Walking => {
                println!("\tCurrently Walking {:?}\n", self.direction);
                match self.direction {
                    Direction::Left => println!("\th: Run Left\n\tk: Slow Down\n\tl: Fall down\n\n\tENTER: Keep walking\n"),
                    Direction::Right => println!("\tk: Run Right\n\th: Slow Down\n\tl: Fall down\n\n\tENTER: Keep walking\n"),
                }
            },
            PlayerState::Running => {
                println!("\tCurrently Running {:?}\n", self.direction);
                match self.direction {
                    Direction::Left => println!("\th: Keep Running Left\n\tk: Slow Down\n\tl: Fall down\n\n\tENTER: Stop paying attention (what's the worst that could happen?)\n"),
                    Direction::Right => println!("\tk: Keep Running Right\n\th: Slow Down\n\tl: Fall down\n\n\tENTER: Stop paying attention (what's the worst that could happen?)\n"),
                }
            },
            PlayerState::Jumping =>println!("\tYou're in the air!\n\n\tl: Back to earth.\n\n\tENTER: Levitate\n"),
            PlayerState::Sitting =>println!("\tCurrently Sitting:\n\n\tj: Stand\n\tl: Lay down\n\n\tENTER: Keep sitting\n"),
            PlayerState::Laying =>println!("\tCurrently Laying Down:\n\n\tj: Sit Up\n\tl: Literally just die\n\n\tENTER: Keep Laying\n"),
            PlayerState::Falling =>println!("\tCurrently Falling! Uh oh:\n\n\tj: Try to catch yourself\n\tl: Don't try to catch yourself (what's the worst that could happen?)\n"),
            _=> {}
        }
    }
}

// Clears the console of output.

fn clear_screen() {
    print!("{}[2J", 27 as char);
}

// Displays current state and game information to the console.

fn render_stats(player: &Player, turns_left: i32) {
    clear_screen();
    //println!("\n-------------------");
    println!("------------------------------------");
    println!("CALORIES BURNT: {}", player.score);
    println!("STAMINA: {}", player.stamina);
    println!("REMAINING TURNS: {}", turns_left );
    println!("HIGH SCORE: {}", player.high_score);
    println!("------------------------------------\n");
    //println!("-------------------\n");
}

// Displays final score and prompts user to continue.

fn end_game(player: &Player) {
    println!("------------------------------------");
    println!("\nFinal Score: {} calories", player.score);  
    println!("Press any key to continue.\n");
    println!("------------------------------------\n");

    let mut input = String::new();
    io::stdin().read_line(&mut input);
}

// Displays the help screen with an overview and the basic controls

fn display_help() {
    clear_screen();
    println!("--------------------------------------------------------------");
    println!("EXERCISE SIMULATOR is a game that finally lets you simulate");
    println!("REAL activity from the COMFORT of your own home. You will be");
    println!("prompted to input actions to keep your EXCERCISE routine going");
    println!("and your goal is to score as many points as possible before");
    println!("running out of TURNS! Or dying! You can do it! Or can you?");
    println!("--------------------------------------------------------------");
    println!("\nGENERAL CONTROLS:\n");
    println!("\t h - LEFT");
    println!("\t j - UP");
    println!("\t k - RIGHT");
    println!("\t l - DOWN");
    println!("\n(Context appropriate options will be displayed each turn)");
    println!("\nPress ENTER to return to menu\n");

    let mut input = String::new();
    io::stdin().read_line(&mut input);
}

// Handles the main game logic. Accepts the current high score for use in the stat display.
// Game is contained within a while loop until the player either surpasses the designated
// winning score, runs out of turns, or dies.

fn game_handler(high_score: i32) -> i32 {

    let game_length = 50;       // set number of turns allowed
    let winning_score = 300;    // change max score possible
    let minimum_effort = -50;   // change lowest score possible

    let mut turn_count = 0;
    let mut game_on = true;

    clear_screen();
    let mut player = Player::new(high_score);
    
    while game_on {

        player.calculate_score();
        render_stats(&player, game_length -turn_count);
        
        player.print_options();
        println!("------------------------------------\n");
        player.handle_input();
        
        turn_count += 1;
        
        if player.stamina < 0 {
            player.state = PlayerState::Dead;
            game_on = false;
        }

        // Check for deadness

        match player.state {
            PlayerState::Dead => {
                game_on = false;
                clear_screen(); 
                println!("You have died.\n");
            },
            _=> {}
        }

        // check for exceeding max turns

        if game_length - turn_count == 0 {
            game_on = false;
            clear_screen(); 
            println!("You've been exercizing too long! Gross! Now you're out of turns!\n");
        
        // check for dropping below minimum effort (score)

        } else if player.score <= minimum_effort {
            game_on = false;
            clear_screen();
            println!("Yeah, exercise sucks. No thanks.");  
        
        // check if player won

        } else if player.score > winning_score {
            game_on = false;
            clear_screen();
            println!("You.. won? at EXERCISE? That's cool I guess.");
        }
    }

    end_game(&player);
    player.score
}

// Handles the main menu

fn main() {
    let mut exec = true;
    let mut first = true;
    let mut input = String::new();
    let mut choice;
    
    let mut score = 0;
    let mut high_score = 0;

    while exec {
        clear_screen();
        input.clear();

        println!("------------------------------------\n");
        println!("    Welcome to EXERCISE SIMULATOR\n");
        println!("------------------------------------\n");
        
        if !first {
            println!("Current high score is {} calories burnt\n", high_score);
        }

        println!("\tNew Game?\n");
        println!("\ty - yes");
        println!("\tn - not yes");
        println!("\th - help screen\n");
        
        println!("------------------------------------\n");

        io::stdin().read_line(&mut input);
        
        if input.len() > 1 {
            input.pop();
            input = input.to_uppercase();
            choice = input.chars().nth(0).unwrap();
            
            print!("{}[2J", 27 as char); // clears console screen
            if choice == 'Y' {
                first = false;
                score = game_handler(high_score);
                if score > high_score {
                    high_score = score; // set high score if applicable
                }
            } else if choice == 'N' {
                    exec = false;
            } else if choice == 'H' {
                display_help();
            } else {
                continue;
            }
        } 
    }

    println!("\nThank you for playing EXERCISE SIMULATOR.\n");
}   