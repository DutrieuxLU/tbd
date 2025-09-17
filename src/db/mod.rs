use colored::Colorize;
use rusqlite::Connection;
pub fn check_database_created(conn: &Connection) -> Option<i32> {
    println!("{}", "Checking Database Creation".magenta());
    None
}
