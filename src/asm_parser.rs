use crate::memory::Memory;
use crate::token::Token;
use crate::util::{
    self, convert_hex_string_to_u8, convert_string_to_u16, convert_string_to_u8, is_zero_page,
};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::mem;

fn populate_string_to_token_table() -> HashMap<&'static str, Token> {
    let mut map = HashMap::new();
    map.insert("LDA", Token::LDA);
    map.insert("LDX", Token::LDX);
    map.insert("LDY", Token::LDY);
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
    //TODO: Add indexed indirect and indirect indexed support
    match special_character {
        '#' => load_immediate_command(found_token, value, mem, curr_mem_add),
        '$' => load_mem_location_command(found_token, value, mem, curr_mem_add),
        _ => println!("default"),
    }
}
fn load_immediate_command(token: Token, value: &str, mem: &mut Memory, curr_mem_add: &mut u16) {
    match token {
        Token::LDA => {
            load_immediate_value(token, value, mem, curr_mem_add);
        }
        Token::LDX => {
            load_immediate_value(token, value, mem, curr_mem_add);
        }
        Token::LDY => {
            load_immediate_value(token, value, mem, curr_mem_add);
        }
        _ => panic!("NO FOUND TOKEN FOR IMMEDIATE COMMAND"),
    }
}
fn load_mem_location_command(token: Token, value: &str, mem: &mut Memory, curr_mem_add: &mut u16) {
    match token {
        Token::LDA => load_memory_location(token, value, curr_mem_add, mem),
        Token::LDX => load_memory_location(token, value, curr_mem_add, mem),
        Token::LDY => load_memory_location(token, value, curr_mem_add, mem),
        _ => panic!("NO FOUND TOKEN FOR MEM LOCATION COMMAND"),
    }
}
fn load_memory_location(token: Token, value: &str, curr_mem_add: &mut u16, mem: &mut Memory) {
    match token {
        Token::LDA => {
            if is_zero_page(value) {
                load_zero_page(Token::LdaZP, value, curr_mem_add, mem)
            } else {
                load_mem_page(Token::LdaAP, value, curr_mem_add, mem);
            }
        }
        Token::LDX => {
            if is_zero_page(value) {
                load_zero_page(Token::LdxZP, value, curr_mem_add, mem)
            } else {
                load_mem_page(Token::LdxAP, value, curr_mem_add, mem);
            }
        }
        Token::LdyZP => {
            if is_zero_page(value) {
                load_zero_page(Token::LdyZP, value, curr_mem_add, mem)
            } else {
                load_mem_page(Token::LdyAP, value, curr_mem_add, mem);
            }
        }
        _ => panic!("NO FOUND TOKEN FOR ZERO PAGE LOADING"),
    }
}

fn load_zero_page(token: Token, value: &str, curr_mem_add: &mut u16, mem: &mut Memory) {
    mem.data[*curr_mem_add as usize] = token as u8;
    *curr_mem_add += 1;
    mem.data[*curr_mem_add as usize] = convert_hex_string_to_u8(value);
    *curr_mem_add += 1;
}

fn load_mem_page(token: Token, value: &str, curr_mem_add: &mut u16, mem: &mut Memory) {
    mem.data[*curr_mem_add as usize] = token as u8;
    *curr_mem_add += 1;
    let h_byte: u8 = convert_hex_string_to_u8(&value[0..1]);
    let l_byte: u8 = convert_string_to_u8(&value[2..3]);
    mem.data[*curr_mem_add as usize] = l_byte;
    *curr_mem_add += 1;
    mem.data[*curr_mem_add as usize] = h_byte;
    *curr_mem_add += 1;
}

fn load_immediate_value(token: Token, value: &str, mem: &mut Memory, curr_mem_add: &mut u16) {
    if is_hex(value) {
        mem.data[*curr_mem_add as usize] = token as u8;
        *curr_mem_add += 1;
        mem.data[*curr_mem_add as usize] = util::convert_hex_string_to_u8(&value[1..]);
        *curr_mem_add += 1;
    } else {
        mem.data[*curr_mem_add as usize] = token as u8;
        *curr_mem_add += 1;
        mem.data[*curr_mem_add as usize] = util::convert_string_to_u8(value);
        *curr_mem_add += 1;
    }
}
fn is_hex(value: &str) -> bool {
    match value.chars().nth(0) {
        Some(c) if c == '$' => {
            return true;
        }
        Some(_) => return false,
        None => panic!("Syntax error for hex"),
    }
}
