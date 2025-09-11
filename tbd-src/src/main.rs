use std::env;

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
    let command = command_opt.expect("command issue");
    match command {
        =>
    }
}
