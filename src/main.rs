use chrono::{DateTime, Local};
use colored::Colorize;
use rusqlite::{Connection, Result, Row};
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
    Remove,
}
pub enum CompletionStatuses {
    Upcoming,
    Complete,
    Late,
    Unknown,
}

impl CompletionStatuses {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Upcoming => "Upcoming",
            Self::Complete => "Complete",
            Self::Late => "Late",
            Self::Unknown => "Unknown",
        }
    }
    pub fn from_str(s: &str) -> Self {
        match s {
            "Upcoming" => Self::Upcoming,
            "Complete" => Self::Complete,
            "Late" => Self::Late,
            _ => Self::Unknown,
        }
    }
}
pub struct Task {
    pub tid: u32,
    pub tname: String,
    pub due_date: Option<String>,
    pub desc: String,
    pub tags: Vec<String>,
    pub c_status: CompletionStatuses,
}
impl Task {
    pub fn new() -> Self {
        Task {
            tid: 0,
            tname: String::new(),
            due_date: Some(Local::now().to_string()),
            desc: String::new(),
            tags: Vec::new(),
            c_status: CompletionStatuses::Upcoming,
        }
    }
    pub fn from_row(row: &Row) -> Result<Task> {
        let tags_str: String = row.get("Tags")?;
        Ok(Task {
            tid: row.get("Tid")?,
            tname: row.get("Name")?,
            due_date: row.get("Due_Date")?,
            desc: row.get("Description")?,
            tags: tags_str.split(',').map(|s| s.to_string()).collect(),
            c_status: CompletionStatuses::from_str(&row.get::<_, String>("Completion_Status")?),
        })
    }
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
        dbg!(&command);
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
            CommandTypes::Remove => operations::tbd_remove(&conn),
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
        "remove" | "Remove" | "r" => Some(CommandTypes::Remove),
        _ => None,
    }
}
