use std::{any::Any, net::IpAddr};

#[warn(unused_imports)]
use clap::Parser;
struct Command {
    pattern: String,
    args: Vec<String>,
}
fn main() {
    let command = std::env::args().nth(2).expect("no command given");
    println!("{:?}", command);
}
