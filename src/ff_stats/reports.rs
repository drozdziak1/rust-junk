extern crate rusqlite;

use rusqlite::{Connection, Statement};
use io;
use io::Write;
use std::process;

// A "last 10 visited websites" query
pub fn ten_last_visits<'a>(conn: &'a Connection) -> Statement {

    conn.prepare(
            "
    SELECT * FROM moz_places
    ORDER BY last_visit_date DESC
    LIMIT 10
    ",
        )
        .unwrap_or_else(|e| {
            writeln!(
                &mut io::stderr(),
                "Failed to connect to your database (caught {:?})",
                e
            ).expect("Could not write to stderr");
            process::exit(1);
        })
}
