use alloc::string::{String, ToString};
use hifive1::{sprint, sprintln};
use rand::prelude::*;

use crate::{current_mtime, read_line};




///* Guessing Game

/// The main game loop 
pub fn game() {
    // let stop = "exit";
    let mut rng = SmallRng::seed_from_u64(current_mtime());
    let picked = number_to_guess(100, &mut rng);
    sprintln!("I've picked a number between 0 and {}", 100);
    loop {

        sprint!("Guess a number: ");
        let guess = parse_guess(&read_guess());
        match guess {
            Some(num) => {
                if num < picked {
                    sprintln!("Too low");
                } else if num > picked {
                    sprintln!("Too high");
                } else {
                    // equal, so we win
                    sprintln!("You win!");
                    break;
                }
            },
            None => {
                sprintln!("Guess wasn't a number!");
            }
        }
    }
}

/// Picks a hard-to-know-beforehand number
fn number_to_guess(max: u8, rng: &mut SmallRng) -> u8 {
    rng.gen_range(0..max)
}

fn read_guess() -> String {
    read_line().trim().to_string()
}

fn parse_guess(guess: &str) -> Option<u8> {
    guess.parse().ok()
}

