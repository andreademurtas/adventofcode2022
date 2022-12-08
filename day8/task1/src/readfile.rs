use std::fs;

pub fn readfile(name: &str) -> String {
    let contents = fs::read_to_string(name).expect("Couldn't read file");
    contents
}
