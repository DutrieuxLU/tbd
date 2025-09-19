use std::io;

use colored::Colorize;
use rusqlite::Connection;

use crate::Task;

pub fn tbd_add(conn: &Connection) {
    let stdin = io::stdin();
    let mut lines = stdin.lines();
    let mut current_task = Task::new();
    println!("Task Name {}", ">>>".bright_blue());
    current_task.tname = lines.next().unwrap().unwrap()
}
pub fn tbd_adjust(conn: &Connection) {}
pub fn tbd_complete(conn: &Connection) {}
pub fn tbd_help(conn: &Connection) {}
pub fn tbd_list(conn: &Connection) {}
