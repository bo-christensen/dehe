pub mod modify;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::exit;
use crate::target::Target;


/// Attempts to get all the .h and .hpp files whether the provided path is a header or
/// a folder. Note: The function is non-recursive
pub fn get_file_list(t: Target) -> Vec<PathBuf>
{
    let mut path_list: Vec<PathBuf> = Vec::new();
    if !t.folder
    {
        let p = PathBuf::from(t.path);
        if is_valid_file_ext(&p)
        {
            path_list.push(p);
        }
    }

    else
    {
        get_files_from_folder(&mut path_list, t.path);
    }

    path_list
}


fn get_files_from_folder(path_list: &mut Vec<PathBuf>, folder: String)
{
    let folder_path = Path::new(&folder);
    if let Ok(files) = fs::read_dir(folder_path)
    {
        for file in files.flatten()
        {
            let file_path = file.path();
            if file_path.is_dir()
            {
                continue;
            }

             if is_valid_file_ext(&file.path())
             {
                     path_list.push(file_path)
             }
        }
    }

    else
    {
        eprintln!("Failed to read contents of folder: {:?}", folder);
        exit(0);
    }
}


fn is_valid_file_ext(path: &PathBuf) -> bool
{
    let valid: bool;
    if let Some(ext) = path.extension()
    {
        valid = ext == "h" || ext == "hpp"
    }

    else
    {
        valid = false;
    }
    valid
}