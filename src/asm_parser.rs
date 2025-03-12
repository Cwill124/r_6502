use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::vec;

use crate::memory::Memory;

pub fn read_asm_file(file_path: String, mem: &mut Memory) {
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return;
        }
    };
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => parse_line(&line, mem),
            Err(e) => eprintln!("Error reading line {}", e),
        }
    }
}

pub fn parse_line(line: &str, mem: &mut Memory) {
    let tokens: Vec<&str> = line.split(" ").collect();
    for token in tokens {}
}
