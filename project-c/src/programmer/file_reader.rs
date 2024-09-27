use std::fs::File;
use std::io::{self, BufRead};

pub fn get_lines(file_path: &String) -> io::Lines<io::BufReader<File>> {
    println!("[CLI] reading file {}", file_path);
    let file = File::open(file_path).expect("Failed to open file");

    let reader = io::BufReader::new(file);

    reader.lines()
}
