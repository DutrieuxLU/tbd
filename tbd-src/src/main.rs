use std::{
    env::{self},
    process,
};

enum CommandTypes {
    Add,
    Complete,
    Adjust,
    List,
    Help,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let command_opt = match args.len() {
        0 => None,
        1 => None,
        _ => {
            dbg!(&args);
            Some(&args[2])
        }
    };
    let command = parse_command(command_opt.expect("command issue"));
    if command.is_none() {
        println!("Error in command parsing! Exiting");
        process::exit(1);
    }
    let command = command.unwrap();
    match command {
        CommandTypes::Add => {}
        CommandTypes::Adjust => {}
        CommandTypes::Complete => {}
        CommandTypes::Help => {}
        CommandTypes::List => {}
    }
}

fn parse_command(command_str: &str) -> Option<CommandTypes> {
    match command_str {
        "add" | "Add" | "a" => Some(CommandTypes::Add),
        _ => None,
    }
}
