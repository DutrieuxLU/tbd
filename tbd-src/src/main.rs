use std::{
    env::{self},
    io::stdin,
    process,
};

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
            CommandTypes::Add => tbd_add(),
            CommandTypes::Adjust => tbd_adjust(),
            CommandTypes::Complete => tbd_complete(),
            CommandTypes::Help => tbd_help(),
            CommandTypes::List => tbd_list(),
        }
    }
}

fn parse_command(command_str: &str) -> Option<CommandTypes> {
    match command_str {
        "add" | "Add" | "a" => Some(CommandTypes::Add),
        _ => None,
    }
}
