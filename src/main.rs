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
    fs::metadata(filename).is_ok()
}

fn write_list_of_files_in_current_directory_to_file() {
    let mut file = fs::File::create(".clean_home").unwrap_or_else(|_| {
        println!("Unable to create .clean_home file");
        std::process::exit(1);
    });

    let paths = fs::read_dir("./").unwrap_or_else(|_| {
        println!("Unable to read current directory");
        std::process::exit(1);
    });

    for path in paths {
        let path = path.unwrap().path();
        let path = path.display().to_string();
        file.write_all(path.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    }
}

fn process_lists<'a>(list_a: Vec<&'a str>, list_b: Vec<&'a str>) -> (Vec<&'a str>, Vec<&'a str>) {
    let mut extra_files: Vec<&str> = Vec::new();
    for file_a in &list_a {
        let mut found = false;
        for file_b in &list_b {
            if file_a == file_b {
                found = true;
                break;
            }
        }
        if !found && !file_a.to_string().is_empty() {
            extra_files.push(file_a);
        }
    }
    extra_files.sort();

    let mut missing_files: Vec<&str> = Vec::new();
    for file_b in &list_b {
        let mut found = false;
        for file_a in &list_a {
            if file_a == file_b {
                found = true;
                break;
            }
        }
        if !found && !file_b.to_string().is_empty() {
            missing_files.push(file_b);
        }
    }
    missing_files.sort();

    (extra_files, missing_files)
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

    println!();

    println!("Missing files:");
    let mut found = false;
    for line in contents.lines() {
        if !line.is_empty() {
            let path = line.to_string();
            if !file_exists(&path) {
                println!("  {}", strip_dot_slash(path));
                found = true;
            }
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
