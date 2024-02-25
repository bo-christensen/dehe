mod uuid;
mod target;
mod file_operations;

use std::process::exit;
use crate::file_operations::{get_file_list, modify};
use crate::file_operations::modify::ModificationResult;
use crate::target::get_entry;

fn main()
{
    let target = get_entry();
    let files = get_file_list(target);
    if files.len() == 0
    {
        eprintln!("The specified path did not contain any valid header files (.h) or (.hpp)");
        exit(0);
    }

    let mut modified_list: Vec<modify::ModificationResult> = Vec::new();
    for path in files
    {
        let modified = file_operations::modify::modify_file(&path);
        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
        modified_list.push(ModificationResult::new(file_name, modified))
    }

    if ! modified_list
        .iter()
        .filter(|f| f.modified)
        .collect::<Vec<_>>()
        .is_empty()
    {
        println!("Modified files:");
        for modified in modified_list
        {
            if modified.modified
            {
                println!("{:?}", modified.name)
            }
        }
    }
}
