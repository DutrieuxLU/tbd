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
pub fn print_dash(conn: &Connection) {}
pub fn print_header(conn: &Connection) {
    println!("{}", TITLE_STR.bright_green());
}
pub fn print_all(conn: &Connection) -> Result<()> {
    let mut table = conn.prepare("SELECT * FROM tasks")?;
    let rows = table.query_map([], |row| {});
}
