use std::{env, io, process, cmp};
use io::Write;
use cmp::Ordering;

extern crate rand;
use rand::*;

mod rps;
use rps::RPS;
use RPS::*;

fn main() {
    let mut generator = rand::thread_rng();

    let player_choice: RPS = if env::args().len() != 2 {
        writeln!(
            &mut io::stderr(),
            "Argument count doesn't add up, choosing randomly..."
        ).expect("Could not write to stderr");

        generator.gen()
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

    let cpu_choice: RPS = generator.gen();

    println!("Player: {:?}", player_choice);
    println!("CPU: {:?}", cpu_choice);
    print!("\n");

    match player_choice.partial_cmp(&cpu_choice).unwrap() {
        Ordering::Less => {
            println!(
                "CPU wins ({:?} superior to {:?})",
                cpu_choice,
                player_choice
            )
        }
        Ordering::Equal => println!("It's a tie! (got {:?} from both)", player_choice),
        Ordering::Greater => {
            println!(
                "Player wins ({:?} superior to {:?})",
                player_choice,
                cpu_choice
            )
        }
    };
}
