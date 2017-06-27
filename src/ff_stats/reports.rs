extern crate rusqlite;
extern crate time;

use std::{process, io};
use io::Write;

use rusqlite::{Connection, Rows};
use time::Timespec;

pub fn report_ten_last_visited<'a>(conn: &'a Connection) -> Rows {

    let mut stmt = match conn.prepare(
        "
    SELECT * FROM moz_places
    ORDER BY last_visit_date DESC
    LIMIT 10
    ",
    ) {
        Ok(stmt) => stmt,
        Err(e) => panic!("Error! {:#?}", e),
    };

    // Execute and return
    stmt.query(&[]).unwrap_or_else(|e| {
        writeln!(&mut io::stderr(), "Query error: {:#?}", e).expect("Could not write to stderr");
        process::exit(1);
    })
}
