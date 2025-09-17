use std::{io::stdin, process};

pub mod operations;
pub mod output;

enum CommandTypes {
    Add,
    Complete,
    Adjust,
    List,
    Help,
}
static PROMPT: &str = "TODO=>";

fn main() {
    let stdin = stdin();
    let mut command_lines = stdin.lines();
    // let args: Vec<String> = env::args().collect();
    loop {
        print!("{PROMPT}");
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
            CommandTypes::Add => operations::tbd_add(&command_opt[1..]),
            CommandTypes::Adjust => operations::tbd_adjust(&command_opt[1..]),
            CommandTypes::Complete => operations::tbd_complete(&command_opt[1..]),
            CommandTypes::Help => operations::tbd_help(&command_opt[1..]),
            CommandTypes::List => operations::tbd_list(&command_opt[1..]),
        }
        output::print_dash();
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
