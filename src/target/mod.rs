use std::{env, fs};
use std::process::exit;


pub struct Target {
    pub path: String,
    pub folder: bool,
}


impl Target {
    pub fn new(p: String, f: bool) -> Self {
        Target {
            path: p,
            folder: f,
        }
    }
}


pub fn get_entry() -> Target {
    let program_arguments: Vec<String> = env::args().collect();
    if program_arguments.len() < 2
    {
        eprintln!("Missing Argument: You must specify a directory or header file");
        exit(0);
    }

    let path_str = &program_arguments[1];
    if !path_exists(&path_str)
    {
        eprintln!("The provided path does not appear to be valid");
        exit(0);
    }

    string_to_target(path_str)
}


fn path_exists(str_path: &String) -> bool {
    fs::metadata(str_path).is_ok()
}


fn string_to_target(path: &String) -> Target {
    let meta = fs::metadata(path);
    match meta {
        Ok(metadata) => {
            Target::new(path.to_owned(), metadata.is_dir())
        }
        Err(_) => {
            eprintln!("The program encountered an unexpected error");
            exit(0);
        }
    }
}