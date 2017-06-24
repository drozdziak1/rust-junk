extern crate rand;

use std::{env, io};
use std::io::Write;
use rand::*;

//TODO do a nice enum choosing below
#[derive(Rand)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

fn main() {
    let user_choice: RPS;
    let generator = rand::thread_rng();

    if env::args().len() != 2 {
        writeln!(
            &mut io::stderr(),
            "Argument count doesn't add up, choosing randomly..."
        ).expect("Could not write to stderr");
        user_choice = generator.gen();
    }

}
