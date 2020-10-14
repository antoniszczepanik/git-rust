use std::env;
use std::fs;
use std::io::ErrorKind;

static HIDDEN_DIR: &str = "git-rust-root";

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    match command.as_str() {
        // no arguments passed
        "init" => init(),
        _ => println!("'{}' is not implemented yet :(", command),
    }
}

fn init() {
    println!("We're initing! :)");
    let dir = fs::create_dir(HIDDEN_DIR);
    match dir {
        Ok(dir) => dir,
        Err(error) => match error.kind() {
            ErrorKind::AlreadyExists => {
                println!("{} has already been initialized.", HIDDEN_DIR);
            },
            other_error => {
                panic!("Problem creating {}: {:?}", HIDDEN_DIR,  other_error);
            }
        },
    };
}
