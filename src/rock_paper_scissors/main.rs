extern crate rand;

use std::{env, io, process};
use std::io::Write;
use rand::*;
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}
use RPS::*;

impl PartialOrd for RPS {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match *self {
            Rock if *other == Scissors => Some(Ordering::Greater),
            Rock if *other == Paper => Some(Ordering::Less),

            Paper if *other == Rock => Some(Ordering::Greater),
            Paper if *other == Scissors => Some(Ordering::Less),

            Scissors if *other == Paper => Some(Ordering::Greater),
            Scissors if *other == Rock => Some(Ordering::Less),

            _ if *self == *other => Some(Ordering::Equal),

            // None doesn't make sense anyway
            _ => {
                panic!(
                    "Cannot handle variant ordering {:?} vs. {:?}, please handle this case",
                    self,
                    other
                )
            }
        }
    }
}

fn main() {
    let variants = &[Rock, Paper, Scissors];
    let mut generator = rand::thread_rng();

    let player_choice: RPS = if env::args().len() != 2 {
        writeln!(
            &mut io::stderr(),
            "Argument count doesn't add up, choosing randomly..."
        ).expect("Could not write to stderr");

        *generator.choose(variants).unwrap()
    } else {
        match env::args().nth(1).unwrap().to_lowercase().as_str() {
            "rock" => Rock,
            "paper" => Paper,
            "scissors" => Scissors,
            _ => {
                writeln!(&mut io::stderr(), "Your choice is invalid")
                    .expect("Could not write to stderr");
                process::exit(1);
            }
        }
    };

    let cpu_choice = *generator.choose(variants).unwrap();

    println!("Player: {:?}", player_choice);
    println!("CPU: {:?}", cpu_choice);
    print!("\n");

    match player_choice.partial_cmp(&cpu_choice).unwrap() {
        Ordering::Less => println!("CPU wins ({:?} superior to {:?})", cpu_choice, player_choice),
        Ordering::Equal => println!("It's a tie! (got two {:?} variants)", player_choice),
        Ordering::Greater => println!("Player wins ({:?} superior to {:?})", player_choice, cpu_choice),
    };
}
