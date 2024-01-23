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
    let paths_string_array: Vec<String> = paths
        .map(|path| path.unwrap().path().display().to_string())
        .collect();

    let contents_array: Vec<&str> = contents.split('\n').collect();
    let paths_array: Vec<&str> = paths_string_array.iter().map(|s| &**s).collect();

    let (extra_files, missing_files) = process_lists(paths_array, contents_array);

    println!("Extra files: ({})", extra_files.len());
    if extra_files.is_empty() {
        println!("  None");
    } else {
        for extra_file in extra_files {
            println!("  - {}", strip_dot_slash(extra_file.to_string()));
        }
    }

    println!();

    println!("Missing files: ({})", missing_files.len());
    if missing_files.is_empty() {
        println!("  None");
    } else {
        for missing_file in missing_files {
            println!("  - {}", strip_dot_slash(missing_file.to_string()));
        }
    }
}

fn print_short_indicator() {
    let paths = fs::read_dir("./").unwrap();
    let paths_string_array: Vec<String> = paths
        .map(|path| path.unwrap().path().display().to_string())
        .collect();

    let mut file = fs::File::open(".clean_home").unwrap_or_else(|_| {
        let num_files = paths_string_array.len();
        println!("{}-0-0", num_files);
        std::process::exit(0);
    });
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let contents_array: Vec<&str> = contents.split('\n').collect();
    let paths_array: Vec<&str> = paths_string_array.iter().map(|s| &**s).collect();

    let (extra_files, missing_files) = process_lists(paths_array.clone(), contents_array);

    println!(
        "{}-{}-{}",
        paths_array.len(),
        extra_files.len(),
        missing_files.len()
    );
}

fn main() {
    let command = std::env::args().nth(1);
    if command == Some("short".to_string()) {
        print_short_indicator();
    } else if command.is_none() {
        if !file_exists(".clean_home") {
            write_list_of_files_in_current_directory_to_file();
        }
        compare_list_of_files_in_current_directory_to_file();
    } else {
        println!("Unknown command: {}", command.unwrap());
    }
}
