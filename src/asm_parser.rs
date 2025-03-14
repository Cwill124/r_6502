use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::ptr::null;
use std::vec;

use crate::memory::Memory;
use crate::token::Token;
use crate::util;

fn populate_string_to_token_table() -> HashMap<&'static str, Token> {
    let mut map = HashMap::new();
    map.insert("LDA", Token::LDA);
    map
}

pub fn read_asm_file(file_path: String, mem: &mut Memory, curr_mem_add: &mut u16) {
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
            Ok(line) => parse_line(&line, mem, curr_mem_add),
            Err(e) => eprintln!("Error reading line {}", e),
        }
    }
}

pub fn parse_line(line: &str, mem: &mut Memory, curr_mem_add: &mut u16) {
    let token_table = populate_string_to_token_table();
    let tokens: Vec<&str> = line.split(" ").collect();
    let amount_of_characters: usize = tokens.len();
    if amount_of_characters == 1 {
        handle_one_character_line();
    } else if amount_of_characters == 2 {
        handle_two_character_line(tokens, mem, token_table, curr_mem_add);
    }
}
fn handle_one_character_line() {}
fn handle_two_character_line(
    tokens: Vec<&str>,
    mem: &mut Memory,
    token_table: HashMap<&str, Token>,
    curr_mem_add: &mut u16,
) {
    let token: &str = tokens[0];
    let found_token: Token;
    let command: &str = tokens[1];
    match token_table.get(token) {
        Some(t) => found_token = t.clone(),
        None => panic!("Syntax error {}", token),
    }
    println!("{:?} is the token", found_token);
    let special_character: char;

    match command.chars().nth(0) {
        Some(c) => special_character = c,
        None => panic!("Error"),
    }
    let value: &str = &command[1..];
    match special_character {
        '#' => load_immediate_command(found_token, value, mem, curr_mem_add),
        _ => println!("default"),
    }
}
fn load_immediate_command(token: Token, value: &str, mem: &mut Memory, curr_mem_add: &mut u16) {
    match token {
        Token::LDA => {
            mem.data[*curr_mem_add as usize] = token as u8;
            *curr_mem_add += 1;
            mem.data[*curr_mem_add as usize] = util::convert_string_to_u8(value);
            *curr_mem_add += 1;
        }
        _ => panic!("NO FOUND TOKEN FOR IMMEDIATE COMMAND"),
    }
}
