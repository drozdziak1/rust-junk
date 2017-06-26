use std::{io, env, process};
use io::Write;

pub fn main() {
    let mut args = env::args();

    let mut current_number = if args.len() != 2 {
        writeln!(
            io::stderr(),
            "Argument count doesn't add up, using starting number 129"
        ).expect("Could not write to stderr");
        129
    } else {
        args.nth(1).unwrap().parse::<u64>().unwrap_or_else(|e| {
            writeln!(
                io::stderr(),
                "Your starting number makes no sense! (Caught {:#?})",
                e
            ).expect("Could not write to stderr");
            process::exit(1);
        })
    };

    let mut steps = 1;
    loop {
        println!("Step: {}, Value: {}", steps, current_number);
        if current_number == 1 {
            break;
        }
        current_number = match current_number % 2 {
            0 => current_number / 2,
            1 => 3 * current_number + 1,
            _ => panic!("WTF?"),
        };
        steps += 1;
    }
}
