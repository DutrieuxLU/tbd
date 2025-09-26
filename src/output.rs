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
            task.due_date.unwrap_or_else(|| "No due date".to_string()),
            task.desc,
            task.tags.join(", "),
            task.c_status.as_str()
        );
    }
}

pub fn print_help() {
    println!("{}", "Usage:".bright_green());
    println!("  tbd [command] [arguments]");
    println!("{}", "\nCommands:".bright_green());
    println!("  add [task name] -d [due date] -t [tags] -s [status] -i [description]    Add a new task");
    println!("  ls                                                                      List all tasks");
    println!("  update [task id] -n [new task name] -d [new due date] -t [new tags] -s [new status] -i [new description]    Update a task");
    println!("  rm [task id]                                                            Remove a task");
    println!("  help                                                                    Show this help message");
}
