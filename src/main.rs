use std::env;
use std::fs;
use std::path::Path;
use std::string::String;


fn main() {
    let args: Vec<String> = env::args().collect();
    let a: Args = Args::new(&args);
    match a.command.as_str() {
        "init" => init(),
        "hash-object" => hash_object(a.filename.as_str()),
        "help" => display_help(),
        _ => println!("Not implemented"),
    }
}

struct Args {
    command: String,
    filename: String,
}

impl Args {
    fn new(args: &[String]) -> Args {
        if args.len() == 2 {
            Args { 
                command: args[1].clone(),
                filename: "".to_string(),
            }
        } else if args.len() == 3 {
            Args { 
                command: args[1].clone(),
                filename: args[2].clone(),
            }
        } else {
            panic!("This number of arguments is not supported");
        }
    }
}


fn init() {
    create_dir(".rgit");
    create_dir(".rgit/objects");
}

fn hash_object(filename: &str) {
   if Path::new(filename).exists() {
        read_file_contents();
   }

}

fn read_file_contents() {
}

fn create_dir(dir_to_create: &str) {
    match fs::create_dir(dir_to_create) {
        Ok(_v) => println!("created dir {}", dir_to_create),
        Err(_e) => println!("error creating {}", dir_to_create),
    };
}

fn display_help() {
    println!("TODO: Help message should be displayed");
}
