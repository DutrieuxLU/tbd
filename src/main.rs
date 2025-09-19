use chrono::{DateTime, Local, Utc};
use colored::Colorize;
use rusqlite::{Connection, Result};
use std::{
    io::{self, Write, stdin},
    process,
};

pub mod db;
pub mod operations;
pub mod output;

static PROMPT: &str = "TODO=> ";

enum CommandTypes {
    Add,
    Complete,
    Adjust,
    List,
    Help,
}
enum CompletionStatuses {
    Upcoming,
    Complete,
    Late,
}

struct Task {
    tid: u32,
    tname: String,
    due_date: DateTime<Local>,
    desc: String,
    tags: Vec<String>,
    c_status: CompletionStatuses,
}

fn main() -> Result<()> {
    let stdin = stdin();
    let mut command_lines = stdin.lines();
    let conn = Connection::open("src/db/todos.db")?;
    let db_created = db::check_database_created(&conn);
    match db_created {
        Err(_) => println!("{}", "Database not created => making now".red().bold()),
        Ok(_) => println!("{}", "Database exists".green()),
    }

    output::print_header(&conn);
    // let args: Vec<String> = env::args().collect();
    loop {
        print!("{PROMPT}");
        let _ = io::stdout().flush();
        let command = command_lines.next().unwrap();
        if command.is_err() {
            println!("Error in command parsing! Exiting");
            process::exit(1);
        }
        let command = parse_command(command.unwrap().as_str()).unwrap();
        match command {
            CommandTypes::Add => operations::tbd_add(&conn),
            CommandTypes::Adjust => operations::tbd_adjust(&conn),
            CommandTypes::Complete => operations::tbd_complete(&conn),
            CommandTypes::Help => operations::tbd_help(&conn),
            CommandTypes::List => operations::tbd_list(&conn),
        }
        output::print_dash(&conn);
    }
}

fn parse_command(command_str: &str) -> Option<CommandTypes> {
    match command_str {
        "add" | "Add" | "a" => Some(CommandTypes::Add),
        "adjust" | "Adjust" | "adj" => Some(CommandTypes::Adjust),
        "complete" | "Complete" | "c" => Some(CommandTypes::Complete),
        "Help" | "help" | "h" => Some(CommandTypes::Help),
        "List" | "list" | "l" => Some(CommandTypes::List),
        _ => None,
    }
}

impl Task {
    pub fn new() -> Self {
        Task {
            tid: 0,
            tname: String::new(),
            due_date: Local::now(),
            desc: String::new(),
            tags: Vec::new(),
            c_status: CompletionStatuses::Upcoming,
        }
    }
}
