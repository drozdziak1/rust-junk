extern crate time;

use std::io;

/**
 * Amaze your friends with this ground-breaking technology!
 */
pub fn main() {
    let birthday: time::Tm;

    let now = time::now();

    println!("Find out the difference between now and any date in seconds!");
    loop {
        let mut input = String::new();

        println!("Give me a \"YYYY-MM-DD\" formatted birthday!");

        io::stdin().read_line(&mut input).expect(
            "Oops! Failed to read stdin!",
        );

        match time::strptime(&input, "%Y-%m-%d") {
            Ok(parsed) => {
                birthday = parsed;
                break;
            }
            Err(e) => println!("Wrong input! Try again! ({:#?})", e),
        };
    }

    println!(
        "Between {} and {},",
        time::strftime("%Y-%m-%d %H:%M:%S", &birthday).unwrap(),
        time::strftime("%Y-%m-%d %H:%M:%S", &now).unwrap()
    );
    println!(
        "There's about {} seconds betweent those dates!",
        (now - birthday).num_seconds().abs()
    );

}
