extern crate rusqlite;

use rusqlite::Connection;
use std::{env, process, io};
use io::Write;

fn main() {
    if env::args().len() < 2 {
        writeln!(
            &mut io::stderr(),
            "Please specify a path to your places.sqlite database"
        ).expect("Could not write to stderr");
        process::exit(1);
    }

    let mut db_path = String::new();

    if let Some(arg) = env::args().nth(1) {
        println!("Next: {}", arg);
        db_path = arg;
    }

    let connection = match Connection::open_with_flags(db_path, rusqlite::SQLITE_OPEN_READ_ONLY) {
        Ok(conn) => conn,
        Err(e) => {
            writeln!(
                &mut io::stderr(),
                "There was a problem when connecting to your db: {:#?}",
                e
            ).expect("Could not write to stderr");
            process::exit(1);
        }
    };

    match connection.execute("SELECT * FROM sqlite_sequence", &[]) {
        Ok(result) => println!("Result {:#?}", result),
        Err(e) => println!("Got error {:#?}", e),
    };

}
