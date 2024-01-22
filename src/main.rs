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

fn main() {
}
