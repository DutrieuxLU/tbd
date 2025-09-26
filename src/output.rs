use crate::Task;
use colored::Colorize;
use rusqlite::{Connection, Result};
static TITLE_STR: &str = "
░██                    ░██                             ░██    ░██████████  ░██████   ░███████     ░██████   
░██                    ░██                             ░██        ░██     ░██   ░██  ░██   ░██   ░██   ░██  
░██         ░██    ░██ ░██    ░██ ░██████    ░███████  ░██        ░██    ░██     ░██ ░██    ░██ ░██     ░██ 
░██         ░██    ░██ ░██   ░██       ░██  ░██                   ░██    ░██     ░██ ░██    ░██ ░██     ░██ 
░██         ░██    ░██ ░███████   ░███████   ░███████             ░██    ░██     ░██ ░██    ░██ ░██     ░██ 
░██         ░██   ░███ ░██   ░██ ░██   ░██         ░██            ░██     ░██   ░██  ░██   ░██   ░██   ░██  
░██████████  ░█████░██ ░██    ░██ ░█████░██  ░███████             ░██      ░██████   ░███████     ░██████   

                                                                                                            ";
pub fn print_dash(conn: &Connection) {
    println!(
        "{}",
        "-----------------------------------------------------------------------------------------------------------------------------------".blue()
    )
}
pub fn print_header(conn: &Connection) {
    println!("{}", TITLE_STR.bright_green());
}
pub fn print_all(conn: &Connection) {
    let mut table = conn.prepare("SELECT * FROM tasks").unwrap();
    let mut rows = table.query_map([], |row| Task::from_row(row)).unwrap();
    for row in rows {
        let task = row.unwrap();
        println!(
            "{} | {} | {} | {} | {} | {}",
            task.tid,
            task.tname,
            task.due_date
                .map(|dt| dt.format("%m/%d/%Y %H:%M").to_string())
                .unwrap_or_else(|| "No due date".to_string()),
            task.desc,
            task.tags.join(", "),
            task.c_status.as_str()
        );
    }
}
