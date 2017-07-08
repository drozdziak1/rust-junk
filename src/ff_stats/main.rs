extern crate rusqlite;
extern crate time;

mod reports;

use rusqlite::Connection;
use std::{env, process, io};
use io::Write;
use time::Timespec;

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
        println!("Looking into database {}", arg);
        db_path = arg;
    }

    let connection = Connection::open_with_flags(db_path, rusqlite::SQLITE_OPEN_READ_ONLY)
        .unwrap_or_else(|e| {
            writeln!(
                &mut io::stderr(),
                "There was a problem when connecting to your db: {:#?}",
                e
            ).expect("Could not write to stderr");
            process::exit(1);
        });

    let mut rows = reports::ten_last_visits(&connection);
    let mut rows = rows.query(&[]).unwrap_or_else(|e| {
        writeln!(&mut io::stderr(), "Query error: {:#?}", e).expect("Could not write to stderr");
        process::exit(1);
    });

    // Work with the results
    let mut i = 1;
    while let Some(Ok(row)) = rows.next() {
        let url: String = row.get("url");

        // Account for sites with empty titles
        let title: String = match row.get_checked("title") {
            Ok(t) => t,
            Err(_) => String::from("<no title>"),
        };

        // Break down the microseconds stored in last_visit_date
        let visit_date_usec: i64 = row.get("last_visit_date");
        let visit_date = &time::at(Timespec::new(visit_date_usec / 1_000_000, 0));
        let visit_date_str = time::strftime("%Y-%m-%d %H:%M:%S", visit_date)
            .unwrap_or_else(|_| String::from("Invalid visit date"));

        println!("#{}\nTitle: {:?}", i, title);
        println!("URL: {:?}", url);
        println!("Visit Date: {:?}\n", visit_date_str);
        i += 1;
    }
}
