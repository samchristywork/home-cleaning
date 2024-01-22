use std::fs;
use std::io::prelude::*;

fn strip_dot_slash(path: String) -> String {
    let mut path = path;
    if path.starts_with("./") {
        path = path[2..].to_string();
    }
    path
}

fn file_exists(filename: &str) -> bool {
    let metadata = fs::metadata(filename);
    match metadata {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn write_list_of_files_in_current_directory_to_file() {
    let mut file = fs::File::create(".clean_home").unwrap();

    let paths = fs::read_dir("./").unwrap();
    for path in paths {
        let path = path.unwrap().path();
        let path = path.display().to_string();
        file.write_all(path.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    }
}

fn compare_list_of_files_in_current_directory_to_file() {
    let paths = fs::read_dir("./").unwrap();
    let mut file = fs::File::open(".clean_home").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    println!("Extra files:");
    let mut found = false;
    for path in paths {
        let path = path.unwrap().path();
        let path = path.display().to_string();
        if !contents.contains(&path) {
            println!("  {}", strip_dot_slash(path));
            found = true;
        }
    }
    if !found {
        println!("  None");
    }
}

fn main() {
    if !file_exists(".clean_home") {
        write_list_of_files_in_current_directory_to_file();
    }
    compare_list_of_files_in_current_directory_to_file();
}
