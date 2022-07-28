extern crate colored;
extern crate rand;

use colored::Colorize;
use rand::Rng;
use std::io::{self, Write};

fn main() {
    println!("Rust Paper Scissors");

    let mut user_score = 0;
    let mut opponent_score = 0;

    loop {
        println!(
            "Options:
        1) Rock
        2) Paper
        3) Scissors
        
        Or type 'q' to quit"
        );

        let user_input = input("Your choice: ");
        let user_play;
        let opponent_input = rand::thread_rng().gen_range(1..4);
        let opponent_play;

        // handling player's choice
        if user_input == "1" || user_input == "2" || user_input == "3" {
            if user_input == "1" {
                user_play = "ROCK";
            } else if user_input == "2" {
                user_play = "PAPER"
            } else {
                user_play = "SCISSOR"
            }
        } else if user_input == "q" {
            println!("Quitting the program: Goodbye...");
            break;
        } else {
            println!("{}", "Invalid option, try again".red());
            continue;
        }

        // handling opponent's choice
        if opponent_input == 1 {
            opponent_play = "ROCK"
        } else if opponent_input == 2 {
            opponent_play = "PAPER"
        } else {
            opponent_play = "SCISSOR"
        }

        println!("You chose {}", user_play.blue());
        println!("Opponent chose {}", opponent_play.red());

        // finding out who won the game
        if user_play == "ROCK" {
            if opponent_play == "PAPER" {
                println!("{}", "You lost".red());
                opponent_score += 1
            } else if opponent_play == "SCISSOR" {
                println!("{}", "You win".green());
                user_score += 1
            } else if opponent_play == user_play {
                println!("Tie")
            }
        } else if user_play == "PAPER" {
            if opponent_play == "SCISSOR" {
                println!("{}", "You lost".red());
                opponent_score += 1
            } else if opponent_play == "ROCK" {
                println!("{}", "You win".green());
                user_score += 1
            } else if opponent_play == user_play {
                println!("Tie")
            }
        } else if user_play == "SCISSOR" {
            if opponent_play == "ROCK" {
                println!("{}", "You lost".red());
                opponent_score += 1
            } else if opponent_play == "PAPER" {
                println!("{}", "You win".green());
                user_score += 1
            } else if opponent_play == user_play {
                println!("Tie")
            }
        }

        println!(
            "Your score {}       Opponent score {}",
            user_score, opponent_score
        );
    }
}

fn input(prompt: &str) -> String {
    print!("{}", prompt);

    let mut string = String::new();

    // prompting user in the same line
    io::stdout().flush().unwrap();

    // getting user input
    io::stdin().read_line(&mut string).unwrap();

    // removing extra lines
    trim_new_line(&mut string);

    // returning the new string
    string
}

fn trim_new_line(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();

        if s.ends_with('\r') {
            s.pop();
        }
    }
}
