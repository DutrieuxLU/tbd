use std::io;

use chrono::{Local, NaiveDateTime, TimeZone};
use colored::Colorize;
use rusqlite::Connection;

use crate::{CompletionStatuses, Task};

pub fn tbd_add(conn: &Connection) {
    let stdin = io::stdin();
    let mut lines = stdin.lines();
    let mut current_task = Task::new();

    // name
    println!("Task Name {}", ">>>".bright_blue());
    current_task.tname = lines.next().unwrap().unwrap();

    // desc
    println!("Description or links {}", ">>>".bright_blue());
    current_task.desc = lines.next().unwrap().unwrap();

    // due date
    println!(
        "Due Date `MM/DD/YYYY HH:MM` Or Leave Empty {}",
        ">>>".bright_blue()
    );
    let dd_str = lines.next().unwrap().unwrap();
    current_task.due_date = if dd_str.is_empty() {
        None
    } else {
        match NaiveDateTime::parse_from_str(dd_str.as_str(), "%m/%d/%Y %H:%M") {
            Ok(naive_dt) => Local.from_local_datetime(&naive_dt).single(),
            Err(_) => {
                println!("Invalid format! Please use MM/DD/YYYY HH:MM");
                None
            }
        }
    };
    // tags
    println!("Any Class/Assignment/Type tags {}", ">>>".bright_blue());
    current_task.tags = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.to_string().to_lowercase())
        .collect();
    let curr_time = Local::now();
    current_task.c_status = match &current_task.due_date {
        Some(dd) => CompletionStatuses::Upcoming,
        None => crate::CompletionStatuses::Unkown,
    }
}
pub fn tbd_adjust(conn: &Connection) {}
pub fn tbd_complete(conn: &Connection) {}
pub fn tbd_help(conn: &Connection) {}
pub fn tbd_list(conn: &Connection) {}
