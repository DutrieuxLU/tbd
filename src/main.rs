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
    due_date: String,
    desc: String,
    tags: Vec<String>,
    c_status: CompletionStatuses,
}

fn main() -> Result<()> {
    let stdin = stdin();
    let mut command_lines = stdin.lines();
    let conn = Connection::open("src/db/todos.db")?;
    db::check_database_created(&conn);
    output::print_header(&conn);
    // let args: Vec<String> = env::args().collect();
    loop {
        print!("{PROMPT}");
        let _ = io::stdout().flush();
        let command_opt: Vec<String> = command_lines
            .next()
            .unwrap()
            .unwrap()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        let command = parse_command(&command_opt[0]);
        if command.is_none() {
            println!("Error in command parsing! Exiting");
            process::exit(1);
        }
        let command = command.unwrap();
        match command {
            CommandTypes::Add => operations::tbd_add(&conn, &command_opt[1..]),
            CommandTypes::Adjust => operations::tbd_adjust(&conn, &command_opt[1..]),
            CommandTypes::Complete => operations::tbd_complete(&conn, &command_opt[1..]),
            CommandTypes::Help => operations::tbd_help(&conn, &command_opt[1..]),
            CommandTypes::List => operations::tbd_list(&conn, &command_opt[1..]),
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
