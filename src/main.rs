use std::env;
use std::process::exit;
use uuid::Uuid;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if  args.len() < 1 {
        println!("Please specify which file to modify");
        exit(0);
    }
    
    let file_to_modify = &args[1];
    // check that the file is a .hpp or .h file
    let mut allowed_files: Vec<String> = Vec::new();
    allowed_files.push(".hpp".to_string());
    allowed_files.push(".h".to_string());
    if !allowed_files.iter().any(|ext| file_to_modify.ends_with(ext)) {
        println!("Invalid file format. Please specify a .hpp or .h file");
        exit(0);
    }
    
    // check that the file actually exists
    if !std::path::Path::new(file_to_modify).exists() {
        println!("The indicated header file does not exist");
        exit(0);
    }
    
    // generate a new uuid
    let new_uuid = Uuid::new_v4();
    // replace dashes with underscores in uuid and store as string
    let new_uuid_string = new_uuid.to_string().replace("-", "_");
    // write uuid to beginning of file while preserving original contents
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(file_to_modify)
        .expect("Failed to open file");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");

    contents = format!("#ifndef u{}\n#define u{}\n\n{}\n\n#endif", new_uuid_string, new_uuid_string, contents);

    file.seek(std::io::SeekFrom::Start(0))
        .expect("Failed to seek to start of file");
    file.write_all(contents.as_bytes())
        .expect("Failed to write to file");
}
