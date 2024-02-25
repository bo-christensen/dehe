use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::process::exit;
use crate::uuid;

pub struct ModificationResult {
    pub name: String,
    pub modified: bool
}


impl ModificationResult {
    pub fn new(name: String, modified: bool) -> Self {
        ModificationResult {
            name,
            modified
        }
    }
}


/// Read a file and write header guards if necessary. Returns true when a file was modified
pub fn modify_file(path: &PathBuf) -> bool {
    let modified_file: bool;


    if let Ok(mut file) = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
    {
        let mut file_content = String::new();
        read_file_contents(&mut file_content, &mut file);
        if should_modify(&file_content) {
            rewrite_file(&file_content, &mut file);
            modified_file = true;
        }

        else
        {
            modified_file = false;
        }
    }

    else
    {
        eprintln!("Operation failed when attempting to update file: {:?}", path.to_str().unwrap());
        modified_file = false
    }

    return modified_file;
}

fn read_file_contents(str: &mut String, file: &mut File) {
    match file.read_to_string(str) {
        Err(_) => {
            eprintln!("Operation failed when attempting to read from file");
            exit(0);
        },
        _ => (),
    }
}

fn should_modify(str: &String) -> bool {
    let matched_pattern = str
        .lines()
        .any(|line| line.starts_with("#ifndef u"));
    !matched_pattern
}


fn rewrite_file(content: &String, file: &mut File) {
    let identifier = uuid::generate_new_uuid();
    let new_content = format!("#ifndef {}\n#define {}\n\n{}\n\n#endif", identifier, identifier, content);
    // ignoring the result: This could technically panic?
    let _ = file.seek(SeekFrom::Start(0));
    match file.write_all(new_content.as_bytes()) {
        Err(_) => {
            eprintln!("Failed to write headers to file!")
        },
        _ => ()
    }
}