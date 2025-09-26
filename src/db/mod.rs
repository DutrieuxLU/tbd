use colored::Colorize;
use rusqlite::{Connection, Result};
pub fn check_database_created(conn: &Connection) -> Result<()> {
    println!("{}", "Checking Database Creation".magenta());
    let _ = conn.execute(
        "
            CREATE TABLE IF NOT EXISTS tasks (
                Tid                INTEGER PRIMARY KEY,
                Name               TEXT NOT NULL,
                Due_Date           DATETIME,
                Description        TEXT,
                Tags               TEXT,
                Completion_Status  BOOLEAN
        );",
        (),
    )?;
    Ok(())
}
