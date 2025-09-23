use colored::Colorize;
use rusqlite::{Connection, Result};
pub fn check_database_created(conn: &Connection) -> Result<()> {
    println!("{}", "Checking Database Creation".magenta());
    let _ = conn.execute(
        "
            CREATE TABLE IF NOT EXISTS tasks (
                Task_ID            INTEGER PRIMARY KEY,
                Name               TEXT NOT NULL,
                Due_Date           DATETIME,
                Description        TEXT,
                Tags               TEXT,
                Completion_Status  BOOLEAN
        );",
        (),
    )?;
    let _ = conn.execute(
        "
            CREATE TABLE task_tags (
                id INTEGER PRIMARY KEY,
                task_id INTEGER NOT NULL,   
                tag TEXT NOT NULL,
                FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE
        );",
        (),
    )?;
    Ok(())
}
