use crate::memory::Memory;
use crate::token::Token;
use crate::util::{self, convert_hex_string_to_u8, is_zero_page};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn populate_string_to_token_table() -> HashMap<&'static str, Token> {
    let mut map = HashMap::new();
    map.insert("LDA", Token::LDA);
    map.insert("LDX", Token::LDX);
    map.insert("LDY", Token::LDY);
    map.insert("ADC", Token::ADC);
    map.insert("STA", Token::STA);
    map.insert("STX", Token::STX);
    map.insert("STY", Token::STY);
    map.insert("JMP", Token::JMP);
    map.insert("JSR", Token::JSR);
    map.insert("AND", Token::AND);
    map.insert("ASL", Token::ASL);
    map.insert("BCC", Token::BCC);
    map.insert("BCS", Token::BCS);
    map.insert("BEQ", Token::BEQ);
    map.insert("BIT", Token::BIT);
    map.insert("BMI", Token::BMI);
    map.insert("BNE", Token::BNE);
    map.insert("BPL", Token::BPL);
    map.insert("BRK", Token::BRK);
    map.insert("BVC", Token::BVC);
    map.insert("BVS", Token::BVS);
    map.insert("CLC", Token::CLC);
    map.insert("CLD", Token::CLD);
    map.insert("CLI", Token::CLI);
    map.insert("CLV", Token::CLV);
    map.insert("CMP", Token::CMP);
    map.insert("CPX", Token::CPX);
    map.insert("CPY", Token::CPY);
    map.insert("DEC", Token::DEC);
    map.insert("DEX", Token::DEX);
    map.insert("DEY", Token::DEY);
    map.insert("EOR", Token::EOR);
    map.insert("INC", Token::INC);
    map.insert("INX", Token::INX);
    map.insert("INY", Token::INY);
    map.insert("LSR", Token::LSR);
    map.insert("NOP", Token::NOP);
    map.insert("ORA", Token::ORA);
    map.insert("PHA", Token::PHA);
    map.insert("PHP", Token::PHP);
    map.insert("PLA", Token::PLA);
    map.insert("PLP", Token::PLP);
    map.insert("ROL", Token::ROL);
    map.insert("ROR", Token::ROR);
    map.insert("RTI", Token::RTI);
    map.insert("RTS", Token::RTS);
    map.insert("SBC", Token::SBC);
    map.insert("SEC", Token::SEC);
    map.insert("SED", Token::SED);
    map.insert("SEI", Token::SEI);
    map.insert("TAX", Token::TAX);
    map.insert("TAY", Token::TAY);
    map.insert("TSX", Token::TSX);
    map.insert("TXA", Token::TXA);
    map.insert("TXS", Token::TXS);
    map.insert("TYA", Token::TYA);
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
    let token_table = populate_string_to_token_table();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                if line.is_empty() {
                    continue;
                } else {
                    parse_line(&line, mem, curr_mem_add, &token_table)
                }
            }
            Err(e) => eprintln!("Error reading line {}", e),
        }
    }
}
fn parse_line(
    line: &str,
    mem: &mut Memory,
    curr_mem_add: &mut u16,
    token_table: &HashMap<&str, Token>,
) {
    let tokens: Vec<&str> = line.split(" ").collect();
    let amount_of_characters: usize = tokens.len();
    if amount_of_characters == 1 {
        handle_one_character_line(tokens[0], mem, token_table, curr_mem_add);
    } else if amount_of_characters == 2 {
        handle_two_character_line(tokens, mem, token_table, curr_mem_add);
    }
}
fn handle_one_character_line(
    token: &str,
    mem: &mut Memory,
    token_table: &HashMap<&str, Token>,
    curr_mem_add: &mut u16,
) {
    let found_token: Token;

    match token_table.get(token) {
        Some(t) => found_token = t.clone(),
        None => panic!("Syntax error {}", token),
    }
    match found_token {
        Token::ASL => load_relative_value(Token::ASL, mem, curr_mem_add),
        Token::BCC => load_relative_value(Token::BCC, mem, curr_mem_add),
        Token::BCS => load_relative_value(Token::BCS, mem, curr_mem_add),
        Token::BEQ => load_relative_value(Token::BEQ, mem, curr_mem_add),
        Token::BMI => load_relative_value(Token::BMI, mem, curr_mem_add),
        Token::BNE => load_relative_value(Token::BNE, mem, curr_mem_add),
        Token::BPL => load_relative_value(Token::BPL, mem, curr_mem_add),
        Token::BRK => load_relative_value(Token::BRK, mem, curr_mem_add),
        Token::BVC => load_relative_value(Token::BVC, mem, curr_mem_add),
        Token::BVS => load_relative_value(Token::BVS, mem, curr_mem_add),
        Token::CLC => load_relative_value(Token::CLC, mem, curr_mem_add),
        Token::CLD => load_relative_value(Token::CLD, mem, curr_mem_add),
        Token::CLI => load_relative_value(Token::CLI, mem, curr_mem_add),
        Token::CLV => load_relative_value(Token::CLV, mem, curr_mem_add),
        Token::DEX => load_relative_value(Token::DEX, mem, curr_mem_add),
        Token::DEY => load_relative_value(Token::DEY, mem, curr_mem_add),
        Token::INX => load_relative_value(Token::INX, mem, curr_mem_add),
        Token::INY => load_relative_value(Token::INY, mem, curr_mem_add),
        Token::LSR => load_relative_value(Token::LSR, mem, curr_mem_add),
        Token::NOP => load_relative_value(Token::NOP, mem, curr_mem_add),
        Token::PHA => load_relative_value(Token::PHA, mem, curr_mem_add),
        Token::PLA => load_relative_value(Token::PLA, mem, curr_mem_add),
        Token::PLP => load_relative_value(Token::PLP, mem, curr_mem_add),
        Token::ROL => load_relative_value(Token::ROL, mem, curr_mem_add),
        Token::ROR => load_relative_value(Token::ROR, mem, curr_mem_add),
        Token::RTI => load_relative_value(Token::RTI, mem, curr_mem_add),
        Token::RTS => load_relative_value(Token::RTS, mem, curr_mem_add),
        Token::SEC => load_relative_value(Token::SEC, mem, curr_mem_add),
        Token::SED => load_relative_value(Token::SED, mem, curr_mem_add),
        Token::SEI => load_relative_value(Token::SEI, mem, curr_mem_add),
        Token::TAX => load_relative_value(Token::TAX, mem, curr_mem_add),
        Token::TAY => load_relative_value(Token::TAY, mem, curr_mem_add),
        Token::TSX => load_relative_value(Token::TSX, mem, curr_mem_add),
        Token::TXA => load_relative_value(Token::TXA, mem, curr_mem_add),
        Token::TXS => load_relative_value(Token::TXS, mem, curr_mem_add),
        Token::TYA => load_relative_value(Token::TYA, mem, curr_mem_add),
        _ => panic!("NO TOKEN FOUND FOR RELATIVE VALUE"),
    }
}

fn handle_two_character_line(
    tokens: Vec<&str>,
    mem: &mut Memory,
    token_table: &HashMap<&str, Token>,
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
        Token::ADC => {
            load_immediate_value(token, value, mem, curr_mem_add);
        }
        Token::AND => {
            load_immediate_value(token, value, mem, curr_mem_add);
        }
        Token::CMP => {
            load_immediate_value(token, value, mem, curr_mem_add);
        }
        Token::CPX => {
            load_immediate_value(token, value, mem, curr_mem_add);
        }
        Token::CPY => {
            load_immediate_value(token, value, mem, curr_mem_add);
        }
        Token::EOR => {
            load_immediate_value(token, value, mem, curr_mem_add);
        }
        Token::ORA => {
            load_immediate_value(token, value, mem, curr_mem_add);
        }
        Token::ROL => {
            load_immediate_value(token, value, mem, curr_mem_add);
        }
        Token::ROR => {
            load_immediate_value(token, value, mem, curr_mem_add);
        }
        Token::SBC => {
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
        Token::ADC => load_memory_location(token, value, curr_mem_add, mem),
        Token::STA => load_memory_location(token, value, curr_mem_add, mem),
        Token::STX => load_memory_location(token, value, curr_mem_add, mem),
        Token::STY => load_memory_location(token, value, curr_mem_add, mem),
        Token::JMP => load_memory_location(token, value, curr_mem_add, mem),
        Token::JSR => load_memory_location(token, value, curr_mem_add, mem),
        Token::AND => load_memory_location(token, value, curr_mem_add, mem),
        Token::ASL => load_memory_location(token, value, curr_mem_add, mem),
        Token::BIT => load_memory_location(token, value, curr_mem_add, mem),
        Token::CMP => load_memory_location(token, value, curr_mem_add, mem),
        Token::CPX => load_memory_location(token, value, curr_mem_add, mem),
        Token::CPY => load_memory_location(token, value, curr_mem_add, mem),
        Token::DEC => load_memory_location(token, value, curr_mem_add, mem),
        Token::EOR => load_memory_location(token, value, curr_mem_add, mem),
        Token::INC => load_memory_location(token, value, curr_mem_add, mem),
        Token::LSR => load_memory_location(token, value, curr_mem_add, mem),
        Token::ORA => load_memory_location(token, value, curr_mem_add, mem),
        Token::ROL => load_memory_location(token, value, curr_mem_add, mem),
        Token::ROR => load_memory_location(token, value, curr_mem_add, mem),
        Token::SBC => load_memory_location(token, value, curr_mem_add, mem),
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
        Token::LDY => {
            if is_zero_page(value) {
                load_zero_page(Token::LdyZP, value, curr_mem_add, mem)
            } else {
                load_mem_page(Token::LdyAP, value, curr_mem_add, mem);
            }
        }
        Token::ADC => {
            if is_zero_page(value) {
                load_zero_page(Token::AdcZP, value, curr_mem_add, mem);
            } else {
                load_mem_page(Token::AdcAP, value, curr_mem_add, mem);
            }
        }
        Token::STA => {
            if is_zero_page(value) {
                load_zero_page(Token::STA, value, curr_mem_add, mem);
            } else {
                load_mem_page(Token::StaAP, value, curr_mem_add, mem);
            }
        }
        Token::STX => {
            if is_zero_page(value) {
                load_zero_page(Token::StxZP, value, curr_mem_add, mem);
            } else {
                load_mem_page(Token::StxAP, value, curr_mem_add, mem);
            }
        }
        Token::STY => {
            if is_zero_page(value) {
                load_zero_page(Token::StyZP, value, curr_mem_add, mem);
            } else {
                load_mem_page(Token::StyAP, value, curr_mem_add, mem);
            }
        }
        Token::JMP => {
            if is_zero_page(value) {
                load_zero_page(Token::JMP, value, curr_mem_add, mem);
            } else {
                load_mem_page(Token::JMP, value, curr_mem_add, mem);
            }
        }
        Token::JSR => {
            if is_zero_page(value) {
                load_zero_page(Token::JSR, value, curr_mem_add, mem);
            } else {
                load_mem_page(Token::JSR, value, curr_mem_add, mem);
            }
        }
        Token::AND => {
            if is_zero_page(value) {
                load_zero_page(Token::AndZP, value, curr_mem_add, mem);
            } else {
                load_mem_page(Token::AndAP, value, curr_mem_add, mem);
            }
        }
        Token::ASL => {
            if is_zero_page(value) {
                load_zero_page(Token::AslZP, value, curr_mem_add, mem);
            } else {
                load_mem_page(Token::AslAP, value, curr_mem_add, mem);
            }
        }
        Token::BIT => {
            if is_zero_page(value) {
                load_zero_page(Token::BIT, value, curr_mem_add, mem);
            } else {
                load_mem_page(Token::BitAP, value, curr_mem_add, mem);
            }
        }
        Token::CMP => {
            if is_zero_page(value) {
                load_zero_page(Token::CmpZP, value, curr_mem_add, mem);
            } else {
                load_mem_page(Token::CmpAP, value, curr_mem_add, mem);
            }
        }
        Token::CPX => {
            if is_zero_page(value) {
                load_zero_page(Token::CpxZP, value, curr_mem_add, mem);
            } else {
                load_mem_page(Token::CpxAP, value, curr_mem_add, mem);
            }
        }
        Token::CPY => {
            if is_zero_page(value) {
                load_zero_page(Token::CpyZP, value, curr_mem_add, mem);
            } else {
                load_mem_page(Token::CpyAP, value, curr_mem_add, mem);
            }
        }
        Token::DEC => {
            if is_zero_page(value) {
                load_zero_page(Token::DEC, value, curr_mem_add, mem);
            } else {
                load_mem_page(Token::DecAP, value, curr_mem_add, mem);
            }
        }
        Token::EOR => {
            if is_zero_page(value) {
                load_zero_page(Token::EorZP, value, curr_mem_add, mem);
            } else {
                load_mem_page(Token::EorAP, value, curr_mem_add, mem);
            }
        }
        Token::INC => {
            if is_zero_page(value) {
                load_zero_page(Token::INC, value, curr_mem_add, mem);
            } else {
                load_mem_page(Token::IncAP, value, curr_mem_add, mem);
            }
        }
        Token::LSR => {
            if is_zero_page(value) {
                load_zero_page(Token::LsrZP, value, curr_mem_add, mem);
            } else {
                load_mem_page(Token::LsrAP, value, curr_mem_add, mem);
            }
        }
        Token::ORA => {
            if is_zero_page(value) {
                load_zero_page(Token::OraZP, value, curr_mem_add, mem);
            } else {
                load_mem_page(Token::OraAP, value, curr_mem_add, mem);
            }
        }
        Token::ROL => {
            if is_zero_page(value) {
                load_zero_page(Token::RolZP, value, curr_mem_add, mem);
            } else {
                load_mem_page(Token::RolAP, value, curr_mem_add, mem);
            }
        }
        Token::ROR => {
            if is_zero_page(value) {
                load_zero_page(Token::RorZP, value, curr_mem_add, mem);
            } else {
                load_mem_page(Token::RorAP, value, curr_mem_add, mem);
            }
        }
        Token::SBC => {
            if is_zero_page(value) {
                load_zero_page(Token::SbcZP, value, curr_mem_add, mem);
            } else {
                load_mem_page(Token::SbcAP, value, curr_mem_add, mem);
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
    let h_byte: u8 = convert_hex_string_to_u8(&value[0..2]);
    let l_byte: u8 = convert_hex_string_to_u8(&value[2..4]);
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

fn load_relative_value(token: Token, mem: &mut Memory, curr_mem_add: &mut u16) {
    mem.data[*curr_mem_add as usize] = token as u8;
    *curr_mem_add += 1;
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
