use crate::memory::Memory;
use crate::token::Token;
use crate::cycle_map;
use crate::util::{self, convert_hex_string_to_u8, is_zero_page};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};


/// Populates a `HashMap` mapping assembly instruction mnemonics to their corresponding `Token` variants.
///
/// This function creates a `HashMap` where each entry associates a string representation of a
/// 6502 assembly instruction (e.g., "LDA", "LDX", "STA") with a corresponding `Token` variant
/// representing the instruction. This can be useful for parsing or interpreting assembly code
/// in the context of a 6502 emulator or assembler.
///
/// # Returns
/// A `HashMap<&'static str, Token>` mapping instruction mnemonics to their `Token` representations.
///
/// # Example
/// ```rust
/// let instruction_map = populate_string_to_token_table();
/// let token = instruction_map.get("LDA");
/// assert_eq!(token, Some(&Token::LDA));
/// ```
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
/// Reads an assembly file, parses each line, and stores the result in memory.
///
/// This function opens the specified assembly file, reads it line by line, and uses the
/// `parse_line` function to process each non-empty line. The parsed instructions are stored
/// in the provided `Memory` instance starting at the memory address specified by `curr_mem_add`.
/// The `token_table` is used to map assembly instruction mnemonics to their corresponding
/// `Token` variants during the parsing process.
///
/// # Parameters
/// - `file_path`: The path to the assembly file to be read.
/// - `mem`: A mutable reference to the `Memory` instance where the parsed instructions will be stored.
/// - `curr_mem_add`: A mutable reference to the current memory address, which is updated as instructions are added.
///
/// # Errors
/// If the file cannot be opened, an error message is printed to `stderr`. If a line cannot be read,
/// an error message is printed for that specific line.
///
/// # Example
/// ```rust
/// let mut memory = Memory::new();
/// let mut current_mem_addr = 0x8000;
/// read_asm_file("program.asm".to_string(), &mut memory, &mut current_mem_addr);
/// ```
pub fn read_asm_file(file_path: String, mem: &mut Memory, curr_mem_add: &mut u16) {
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return;
        }
    };
    let token_table = populate_string_to_token_table();
    let token_cycle_table = cycle_map::init();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                if line.is_empty() {
                    continue;
                } else {
                    parse_line(&line, mem, curr_mem_add, &token_table, &token_cycle_table)
                }
            }
            Err(e) => eprintln!("Error reading line {}", e),
        }
    }
}
/// Parses a line of assembly code and processes it based on the number of tokens.
///
/// This function splits the provided line into tokens and determines how to process it based on
/// the number of tokens. If the line contains one token, it is handled by `handle_one_character_line`;
/// if it contains two tokens, it is processed by `handle_two_character_line`. The function modifies
/// the memory (`mem`) starting at the current memory address (`curr_mem_add`), updating the memory as
/// instructions are parsed. The `token_table` is used to map assembly instruction mnemonics to their
/// corresponding `Token` variants during parsing.
///
/// # Parameters
/// - `line`: The line of assembly code to be parsed, typically in string form.
/// - `mem`: A mutable reference to the `Memory` instance where the parsed instructions will be stored.
/// - `curr_mem_add`: A mutable reference to the current memory address, which is updated as instructions
///   are parsed and stored.
/// - `token_table`: A reference to a `HashMap` that maps instruction mnemonics to their respective `Token`
///   variants for correct parsing.
///
/// # Behavior
/// - If the line contains one token, it is processed using the `handle_one_character_line` function.
/// - If the line contains two tokens, it is processed using the `handle_two_character_line` function.
///
/// # Example
/// ```rust
/// let mut memory = Memory::new();
/// let mut current_mem_addr = 0x8000;
/// let token_table = populate_string_to_token_table();
/// parse_line("LDA #10", &mut memory, &mut current_mem_addr, &token_table);
/// ```
fn parse_line(
    line: &str,
    mem: &mut Memory,
    curr_mem_add: &mut u16,
    token_table: &HashMap<&str, Token>,
    token_cycle_table : &HashMap<Token,u8>
) {
    let tokens: Vec<&str> = line.split(" ").collect();
    let amount_of_characters: usize = tokens.len();
    if amount_of_characters == 1 {
        handle_one_character_line(tokens[0], mem, token_table, curr_mem_add,token_cycle_table);
    } else if amount_of_characters == 2 {
        handle_two_character_line(tokens, mem, token_table, curr_mem_add,token_cycle_table);
    }
}

/// Handles a single-token line by parsing the token and loading the corresponding instruction into memory.
///
/// This function takes a single assembly instruction token (e.g., "ASL", "BCC", "NOP") and attempts
/// to look it up in the provided `token_table`. If the token is found, it processes the instruction
/// by calling the `load_relative_value` function with the corresponding `Token` and stores it in memory.
/// If the token is not found in the `token_table`, a panic with a syntax error message is triggered.
///
/// The function specifically handles instructions that are expected to have a relative value, updating
/// the memory and current memory address (`curr_mem_add`) as the instructions are processed.
///
/// # Parameters
/// - `token`: The assembly instruction token (e.g., "ASL", "BCC", etc.) to be processed.
/// - `mem`: A mutable reference to the `Memory` instance where the parsed instruction will be stored.
/// - `token_table`: A reference to the `HashMap` that maps instruction mnemonics to their respective `Token` variants.
/// - `curr_mem_add`: A mutable reference to the current memory address, which is updated as the instruction is stored.
///
/// # Panics
/// - If the token is not found in the `token_table`, the function panics with a syntax error message.
/// - If no token is found for the instruction requiring a relative value, it panics with an error message.
///
/// # Example
/// ```rust
/// let mut memory = Memory::new();
/// let mut current_mem_addr = 0x8000;
/// let token_table = populate_string_to_token_table();
/// handle_one_character_line("LDA", &mut memory, &token_table, &mut current_mem_addr);
/// ```
fn handle_one_character_line(
    token: &str,
    mem: &mut Memory,
    token_table: &HashMap<&str, Token>,
    curr_mem_add: &mut u16,
    token_cycle_table : &HashMap<Token,u8>
) {
    let found_token: Token;

    match token_table.get(token) {
        Some(t) => found_token = t.clone(),
        None => panic!("Syntax error {}", token),
    }
    match found_token {
        Token::ASL => load_relative_value(Token::ASL, mem, curr_mem_add,token_cycle_table),
        Token::BCC => load_relative_value(Token::BCC, mem, curr_mem_add,token_cycle_table),
        Token::BCS => load_relative_value(Token::BCS, mem, curr_mem_add,token_cycle_table),
        Token::BEQ => load_relative_value(Token::BEQ, mem, curr_mem_add,token_cycle_table),
        Token::BMI => load_relative_value(Token::BMI, mem, curr_mem_add,token_cycle_table),
        Token::BNE => load_relative_value(Token::BNE, mem, curr_mem_add,token_cycle_table),
        Token::BPL => load_relative_value(Token::BPL, mem, curr_mem_add,token_cycle_table),
        Token::BRK => load_relative_value(Token::BRK, mem, curr_mem_add,token_cycle_table),
        Token::BVC => load_relative_value(Token::BVC, mem, curr_mem_add,token_cycle_table),
        Token::BVS => load_relative_value(Token::BVS, mem, curr_mem_add,token_cycle_table),
        Token::CLC => load_relative_value(Token::CLC, mem, curr_mem_add,token_cycle_table),
        Token::CLD => load_relative_value(Token::CLD, mem, curr_mem_add,token_cycle_table),
        Token::CLI => load_relative_value(Token::CLI, mem, curr_mem_add,token_cycle_table),
        Token::CLV => load_relative_value(Token::CLV, mem, curr_mem_add,token_cycle_table),
        Token::DEX => load_relative_value(Token::DEX, mem, curr_mem_add,token_cycle_table),
        Token::DEY => load_relative_value(Token::DEY, mem, curr_mem_add,token_cycle_table),
        Token::INX => load_relative_value(Token::INX, mem, curr_mem_add,token_cycle_table),
        Token::INY => load_relative_value(Token::INY, mem, curr_mem_add,token_cycle_table),
        Token::LSR => load_relative_value(Token::LSR, mem, curr_mem_add,token_cycle_table),
        Token::NOP => load_relative_value(Token::NOP, mem, curr_mem_add,token_cycle_table),
        Token::PHA => load_relative_value(Token::PHA, mem, curr_mem_add,token_cycle_table),
        Token::PLA => load_relative_value(Token::PLA, mem, curr_mem_add,token_cycle_table),
        Token::PLP => load_relative_value(Token::PLP, mem, curr_mem_add,token_cycle_table),
        Token::ROL => load_relative_value(Token::ROL, mem, curr_mem_add,token_cycle_table),
        Token::ROR => load_relative_value(Token::ROR, mem, curr_mem_add,token_cycle_table),
        Token::RTI => load_relative_value(Token::RTI, mem, curr_mem_add,token_cycle_table),
        Token::RTS => load_relative_value(Token::RTS, mem, curr_mem_add,token_cycle_table),
        Token::SEC => load_relative_value(Token::SEC, mem, curr_mem_add,token_cycle_table),
        Token::SED => load_relative_value(Token::SED, mem, curr_mem_add,token_cycle_table),
        Token::SEI => load_relative_value(Token::SEI, mem, curr_mem_add,token_cycle_table),
        Token::TAX => load_relative_value(Token::TAX, mem, curr_mem_add,token_cycle_table),
        Token::TAY => load_relative_value(Token::TAY, mem, curr_mem_add,token_cycle_table),
        Token::TSX => load_relative_value(Token::TSX, mem, curr_mem_add,token_cycle_table),
        Token::TXA => load_relative_value(Token::TXA, mem, curr_mem_add,token_cycle_table),
        Token::TXS => load_relative_value(Token::TXS, mem, curr_mem_add,token_cycle_table),
        Token::TYA => load_relative_value(Token::TYA, mem, curr_mem_add,token_cycle_table),
        _ => panic!("NO TOKEN FOUND FOR RELATIVE VALUE"),
    }
}

/// Handles a two-token line by parsing the first token and processing the second token (command).
///
/// This function processes a line of assembly code consisting of two tokens: the first token is
/// an instruction mnemonic (e.g., "LDA", "ADC"), and the second token contains additional data
/// (e.g., immediate value or memory address). The function looks up the instruction token in the
/// `token_table` and processes the command (e.g., immediate value or memory location) based on the
/// first character of the second token. If the second token starts with `#`, the function treats it
/// as an immediate value, and if it starts with `$`, it treats it as a memory location. The function
/// calls the appropriate helper functions to load these values into memory and update the memory address.
///
/// # Parameters
/// - `tokens`: A vector of two string slices, the first being the instruction mnemonic and the second being the command.
/// - `mem`: A mutable reference to the `Memory` instance where the parsed instructions and values will be stored.
/// - `token_table`: A reference to the `HashMap` mapping instruction mnemonics to their respective `Token` variants.
/// - `curr_mem_add`: A mutable reference to the current memory address, which is updated as the instruction is stored.
///
/// # Panics
/// - If the instruction token is not found in the `token_table`, a syntax error panic is triggered.
/// - If there is no character in the command string or the command does not start with a valid character, it panics.
///
/// # Behavior
/// - If the command starts with `#`, it is treated as an immediate value and passed to `load_immediate_command`.
/// - If the command starts with `$`, it is treated as a memory location and passed to `load_mem_location_command`.
/// - If the command does not start with `#` or `$`, it is ignored and a default message is printed.
///
/// # Example
/// ```rust
/// let mut memory = Memory::new();
/// let mut current_mem_addr = 0x8000;
/// let token_table = populate_string_to_token_table();
/// let tokens = vec!["LDA", "#$10"];
/// handle_two_character_line(tokens, &mut memory, &token_table, &mut current_mem_addr);
/// ```
fn handle_two_character_line(
    tokens: Vec<&str>,
    mem: &mut Memory,
    token_table: &HashMap<&str, Token>,
    curr_mem_add: &mut u16,
    token_cycle_table : &HashMap<Token,u8>
) {
    let token: &str = tokens[0];
    let found_token: Token;
    let command: &str = tokens[1];
    match token_table.get(token) {
        Some(t) => found_token = t.clone(),
        None => panic!("Syntax error {}", token),
    }
    let special_character: char;

    match command.chars().nth(0) {
        Some(c) => special_character = c,
        None => panic!("Error"),
    }
    let value: &str = &command[1..];
    match special_character {
        '#' => load_immediate_command(found_token, value, mem, curr_mem_add,token_cycle_table),
        '$' => load_mem_location_command(found_token, value, mem, curr_mem_add,token_cycle_table),
        _ => println!("default"),
    }
}

/// Handles the execution of immediate value loading commands based on the given token.
///
/// This function matches the provided `token` to the appropriate instruction type (e.g., LDA, LDX,
/// ADC, etc.), and then loads the immediate value into memory by calling `load_immediate_value`
/// with the provided token, value, and memory reference. The function performs actions based on
/// the specific command to load the immediate value accordingly.
///
/// # Parameters
/// - `token`: A `Token` representing the instruction type (e.g., `LDA`, `LDX`, `ADC`, etc.).
/// - `value`: A string representing the immediate value to be loaded.
/// - `mem`: A mutable reference to the `Memory` structure where the immediate value will be stored.
/// - `curr_mem_add`: A mutable reference to the current memory address that will be updated during the operation.
///
/// # Panics
/// This function will panic if an unsupported token is encountered that does not match any of the predefined
/// command tokens (such as `LDA`, `LDX`, `ADC`, etc.).
///
/// # Example
/// ```rust
/// let mut mem = Memory::new();
/// let mut curr_mem_add = 0x00u16;
/// load_immediate_command(Token::LDA, "0xFF", &mut mem, &mut curr_mem_add);
/// ```
fn load_immediate_command(token: Token, value: &str, mem: &mut Memory, curr_mem_add: &mut u16,token_cycle_table : &HashMap<Token,u8>) {
    match token {
        Token::LDA => {
            load_immediate_value(token, value, mem, curr_mem_add,token_cycle_table);
        }
        Token::LDX => {
            load_immediate_value(token, value, mem, curr_mem_add,token_cycle_table);
        }
        Token::LDY => {
            load_immediate_value(token, value, mem, curr_mem_add,token_cycle_table);
        }
        Token::ADC => {
            load_immediate_value(token, value, mem, curr_mem_add,token_cycle_table);
        }
        Token::AND => {
            load_immediate_value(token, value, mem, curr_mem_add,token_cycle_table);
        }
        Token::CMP => {
            load_immediate_value(token, value, mem, curr_mem_add,token_cycle_table);
        }
        Token::CPX => {
            load_immediate_value(token, value, mem, curr_mem_add,token_cycle_table);
        }
        Token::CPY => {
            load_immediate_value(token, value, mem, curr_mem_add,token_cycle_table);
        }
        Token::EOR => {
            load_immediate_value(token, value, mem, curr_mem_add,token_cycle_table);
        }
        Token::ORA => {
            load_immediate_value(token, value, mem, curr_mem_add,token_cycle_table);
        }
        Token::ROL => {
            load_immediate_value(token, value, mem, curr_mem_add,token_cycle_table);
        }
        Token::ROR => {
            load_immediate_value(token, value, mem, curr_mem_add,token_cycle_table);
        }
        Token::SBC => {
            load_immediate_value(token, value, mem, curr_mem_add,token_cycle_table);
        }
        _ => panic!("NO FOUND TOKEN FOR IMMEDIATE COMMAND"),
    }
}

/// Handles the execution of memory location loading commands based on the given token.
///
/// This function matches the provided `token` to the appropriate instruction type (e.g., LDA, STA,
/// CMP, etc.) and loads or stores values from or to a memory location by calling `load_memory_location`
/// with the provided token, value, current memory address, and memory reference. The function supports a
/// wide variety of commands that involve memory locations.
///
/// # Parameters
/// - `token`: A `Token` representing the instruction type (e.g., `LDA`, `STA`, `AND`, etc.).
/// - `value`: A string representing the memory address or other relevant value to be used with the instruction.
/// - `mem`: A mutable reference to the `Memory` structure where values will be loaded, stored, or processed.
/// - `curr_mem_add`: A mutable reference to the current memory address, which may be updated during the operation.
///
/// # Panics
/// This function will panic if an unsupported token is encountered that does not match any of the predefined
/// command tokens (such as `LDA`, `STA`, `ADC`, etc.).
///
/// # Example
/// ```rust
/// let mut mem = Memory::new();
/// let mut curr_mem_add = 0x1000u16;
/// load_mem_location_command(Token::LDA, "0xFF00", &mut mem, &mut curr_mem_add);
/// ```
fn load_mem_location_command(token: Token, value: &str, mem: &mut Memory, curr_mem_add: &mut u16,token_cycle_table : &HashMap<Token,u8>) {
    match token {
        Token::LDA => load_memory_location(token, value, curr_mem_add, mem,token_cycle_table),
        Token::LDX => load_memory_location(token, value, curr_mem_add, mem,token_cycle_table),
        Token::LDY => load_memory_location(token, value, curr_mem_add, mem,token_cycle_table),
        Token::ADC => load_memory_location(token, value, curr_mem_add, mem,token_cycle_table),
        Token::STA => load_memory_location(token, value, curr_mem_add, mem,token_cycle_table),
        Token::STX => load_memory_location(token, value, curr_mem_add, mem,token_cycle_table),
        Token::STY => load_memory_location(token, value, curr_mem_add, mem,token_cycle_table),
        Token::JMP => load_memory_location(token, value, curr_mem_add, mem,token_cycle_table),
        Token::JSR => load_memory_location(token, value, curr_mem_add, mem,token_cycle_table),
        Token::AND => load_memory_location(token, value, curr_mem_add, mem,token_cycle_table),
        Token::ASL => load_memory_location(token, value, curr_mem_add, mem,token_cycle_table),
        Token::BIT => load_memory_location(token, value, curr_mem_add, mem,token_cycle_table),
        Token::CMP => load_memory_location(token, value, curr_mem_add, mem,token_cycle_table),
        Token::CPX => load_memory_location(token, value, curr_mem_add, mem,token_cycle_table),
        Token::CPY => load_memory_location(token, value, curr_mem_add, mem,token_cycle_table),
        Token::DEC => load_memory_location(token, value, curr_mem_add, mem,token_cycle_table),
        Token::EOR => load_memory_location(token, value, curr_mem_add, mem,token_cycle_table),
        Token::INC => load_memory_location(token, value, curr_mem_add, mem,token_cycle_table),
        Token::LSR => load_memory_location(token, value, curr_mem_add, mem,token_cycle_table),
        Token::ORA => load_memory_location(token, value, curr_mem_add, mem,token_cycle_table),
        Token::ROL => load_memory_location(token, value, curr_mem_add, mem,token_cycle_table),
        Token::ROR => load_memory_location(token, value, curr_mem_add, mem,token_cycle_table),
        Token::SBC => load_memory_location(token, value, curr_mem_add, mem,token_cycle_table),
        _ => panic!("NO FOUND TOKEN FOR MEM LOCATION COMMAND"),
    }
}

/// Handles the loading and storing of values from/to memory locations based on the given token.
///
/// This function processes the provided `token` (such as `LDA`, `STA`, `ADC`, etc.) and determines if
/// the operation involves a zero-page memory address or a full memory page address. It then calls
/// either `load_zero_page` or `load_mem_page` depending on whether the `value` refers to a zero-page
/// address or a larger memory address.
///
/// # Parameters
/// - `token`: A `Token` representing the instruction type (e.g., `LDA`, `STA`, `ADC`, etc.) which dictates the operation.
/// - `value`: A string representing the memory address or value to be used in the operation.
/// - `curr_mem_add`: A mutable reference to the current memory address, which may be updated during the operation.
/// - `mem`: A mutable reference to the `Memory` structure where the operation will be performed.
///
/// # Panics
/// This function will panic if an unsupported token is encountered that does not match any of the predefined
/// command tokens (such as `LDA`, `STA`, `ADC`, etc.) or if the token does not map to a valid operation for zero-page
/// or full memory loading.
///
/// # Example
/// ```rust
/// let mut mem = Memory::new();
/// let mut curr_mem_add = 0x1000u16;
/// load_memory_location(Token::LDA, "0x00FF", &mut curr_mem_add, &mut mem);
/// ```
fn load_memory_location(token: Token, value: &str, curr_mem_add: &mut u16, mem: &mut Memory,token_cycle_table : &HashMap<Token,u8>) {
    match token {
        Token::LDA => {
            if is_zero_page(value) {
                load_zero_page(Token::LdaZP, value, curr_mem_add, mem,token_cycle_table)
            } else {
                load_mem_page(Token::LdaAP, value, curr_mem_add, mem,token_cycle_table);
            }
        }
        Token::LDX => {
            if is_zero_page(value) {
                load_zero_page(Token::LdxZP, value, curr_mem_add, mem,token_cycle_table)
            } else {
                load_mem_page(Token::LdxAP, value, curr_mem_add, mem,token_cycle_table);
            }
        }
        Token::LDY => {
            if is_zero_page(value) {
                load_zero_page(Token::LdyZP, value, curr_mem_add, mem,token_cycle_table)
            } else {
                load_mem_page(Token::LdyAP, value, curr_mem_add, mem,token_cycle_table);
            }
        }
        Token::ADC => {
            if is_zero_page(value) {
                load_zero_page(Token::AdcZP, value, curr_mem_add, mem,token_cycle_table);
            } else {
                load_mem_page(Token::AdcAP, value, curr_mem_add, mem,token_cycle_table);
            }
        }
        Token::STA => {
            if is_zero_page(value) {
                load_zero_page(Token::STA, value, curr_mem_add, mem,token_cycle_table);
            } else {
                load_mem_page(Token::StaAP, value, curr_mem_add, mem,token_cycle_table);
            }
        }
        Token::STX => {
            if is_zero_page(value) {
                load_zero_page(Token::STX, value, curr_mem_add, mem,token_cycle_table);
            } else {
                load_mem_page(Token::StxAP, value, curr_mem_add, mem,token_cycle_table);
            }
        }
        Token::STY => {
            if is_zero_page(value) {
                load_zero_page(Token::STY, value, curr_mem_add, mem,token_cycle_table);
            } else {
                load_mem_page(Token::StyAP, value, curr_mem_add, mem,token_cycle_table);
            }
        }
        Token::JMP => load_mem_page(Token::JMP, value, curr_mem_add, mem,token_cycle_table),
        Token::JSR => load_mem_page(Token::JSR, value, curr_mem_add, mem,token_cycle_table),
        Token::AND => {
            if is_zero_page(value) {
                load_zero_page(Token::AndZP, value, curr_mem_add, mem,token_cycle_table);
            } else {
                load_mem_page(Token::AndAP, value, curr_mem_add, mem,token_cycle_table);
            }
        }
        Token::ASL => {
            if is_zero_page(value) {
                load_zero_page(Token::AslZP, value, curr_mem_add, mem,token_cycle_table);
            } else {
                load_mem_page(Token::AslAP, value, curr_mem_add, mem,token_cycle_table);
            }
        }
        Token::BIT => {
            if is_zero_page(value) {
                load_zero_page(Token::BIT, value, curr_mem_add, mem,token_cycle_table);
            } else {
                load_mem_page(Token::BitAP, value, curr_mem_add, mem,token_cycle_table);
            }
        }
        Token::CMP => {
            if is_zero_page(value) {
                load_zero_page(Token::CmpZP, value, curr_mem_add, mem,token_cycle_table);
            } else {
                load_mem_page(Token::CmpAP, value, curr_mem_add, mem,token_cycle_table);
            }
        }
        Token::CPX => {
            if is_zero_page(value) {
                load_zero_page(Token::CpxZP, value, curr_mem_add, mem,token_cycle_table);
            } else {
                load_mem_page(Token::CpxAP, value, curr_mem_add, mem,token_cycle_table);
            }
        }
        Token::CPY => {
            if is_zero_page(value) {
                load_zero_page(Token::CpyZP, value, curr_mem_add, mem,token_cycle_table);
            } else {
                load_mem_page(Token::CpyAP, value, curr_mem_add, mem,token_cycle_table);
            }
        }
        Token::DEC => {
            if is_zero_page(value) {
                load_zero_page(Token::DEC, value, curr_mem_add, mem,token_cycle_table);
            } else {
                load_mem_page(Token::DecAP, value, curr_mem_add, mem,token_cycle_table);
            }
        }
        Token::EOR => {
            if is_zero_page(value) {
                load_zero_page(Token::EorZP, value, curr_mem_add, mem,token_cycle_table);
            } else {
                load_mem_page(Token::EorAP, value, curr_mem_add, mem,token_cycle_table);
            }
        }
        Token::INC => {
            if is_zero_page(value) {
                load_zero_page(Token::INC, value, curr_mem_add, mem,token_cycle_table);
            } else {
                load_mem_page(Token::IncAP, value, curr_mem_add, mem,token_cycle_table);
            }
        }
        Token::LSR => {
            if is_zero_page(value) {
                load_zero_page(Token::LsrZP, value, curr_mem_add, mem,token_cycle_table);
            } else {
                load_mem_page(Token::LsrAP, value, curr_mem_add, mem,token_cycle_table);
            }
        }
        Token::ORA => {
            if is_zero_page(value) {
                load_zero_page(Token::OraZP, value, curr_mem_add, mem,token_cycle_table);
            } else {
                load_mem_page(Token::OraAP, value, curr_mem_add, mem,token_cycle_table);
            }
        }
        Token::ROL => {
            if is_zero_page(value) {
                load_zero_page(Token::RolZP, value, curr_mem_add, mem,token_cycle_table);
            } else {
                load_mem_page(Token::RolAP, value, curr_mem_add, mem,token_cycle_table);
            }
        }
        Token::ROR => {
            if is_zero_page(value) {
                load_zero_page(Token::RorZP, value, curr_mem_add, mem,token_cycle_table);
            } else {
                load_mem_page(Token::RorAP, value, curr_mem_add, mem,token_cycle_table);
            }
        }
        Token::SBC => {
            if is_zero_page(value) {
                load_zero_page(Token::SbcZP, value, curr_mem_add, mem,token_cycle_table);
            } else {
                load_mem_page(Token::SbcAP, value, curr_mem_add, mem,token_cycle_table);
            }
        }

        _ => panic!("NO FOUND TOKEN FOR ZERO PAGE LOADING"),
    }
}

/// Loads a value from a zero-page memory address based on the provided token and value.
///
/// This function stores a byte value corresponding to the provided `token` at the current memory address
/// in the `Memory` structure, then increments the current memory address. It also stores the byte value
/// corresponding to the provided `value` (converted from a hex string) in the next memory location.
///
/// # Parameters
/// - `token`: A `Token` representing an operation (such as `LDA`, `STA`, etc.). The token is cast to a `u8`
///   value and stored in the current memory location.
/// - `value`: A string representing the hex value to be loaded into memory (e.g., `"FF"`). This value is converted
///   from the hex string to a `u8` and stored in the next memory location.
/// - `curr_mem_add`: A mutable reference to the current memory address. It is updated after each operation.
/// - `mem`: A mutable reference to the `Memory` structure where the values are written to.
///
/// # Example
/// ```rust
/// let mut mem = Memory::new();
/// let mut curr_mem_add = 0x1000u16;
/// load_zero_page(Token::LDA, "FF", &mut curr_mem_add, &mut mem);
/// ```
///
/// This will store the byte corresponding to the `LDA` token in `mem.data[0x1000]`,
/// and the value `0xFF` (from the hex string `"FF"`) in `mem.data[0x1001]`.
fn load_zero_page(token: Token, value: &str, curr_mem_add: &mut u16, mem: &mut Memory,token_cycle_table : &HashMap<Token,u8>) {
    match token_cycle_table.get(&token) {
        Some(num) => mem.data_cycle_count += *num as u32,
        None => panic!("Cycle Error"),
    }
    mem.data[*curr_mem_add as usize] = token as u8;
    *curr_mem_add += 1;
    mem.data[*curr_mem_add as usize] = convert_hex_string_to_u8(value);
    *curr_mem_add += 1;
}

/// Loads a value from a memory address page based on the provided token and value.
///
/// This function stores a byte value corresponding to the provided `token` at the current memory address
/// in the `Memory` structure, then increments the current memory address. It then stores the low and high bytes
/// of the `value` (converted from the hex string) in the next two memory locations, in little-endian order.
///
/// # Parameters
/// - `token`: A `Token` representing an operation (such as `LDA`, `STA`, etc.). The token is cast to a `u8`
///   value and stored in the current memory location.
/// - `value`: A string representing the hex value to be loaded into memory (e.g., `"FF01"`). This value is split
///   into two parts, with the low byte and high byte extracted and stored in little-endian order.
/// - `curr_mem_add`: A mutable reference to the current memory address. It is updated after each operation.
/// - `mem`: A mutable reference to the `Memory` structure where the values are written to.
///
/// # Example
/// ```rust
/// let mut mem = Memory::new();
/// let mut curr_mem_add = 0x1000u16;
/// load_mem_page(Token::LDA, "FF01", &mut curr_mem_add, &mut mem);
/// ```
///
/// This will store the byte corresponding to the `LDA` token in `mem.data[0x1000]`,
/// the low byte `0x01` in `mem.data[0x1001]`, and the high byte `0xFF` in `mem.data[0x1002]`.
///
/// # Note
/// The `value` string is expected to have at least four characters, as it represents a 16-bit value (two bytes).
fn load_mem_page(token: Token, value: &str, curr_mem_add: &mut u16, mem: &mut Memory,token_cycle_table : &HashMap<Token,u8>) {
    match token_cycle_table.get(&token) {
        Some(num) => mem.data_cycle_count += *num as u32,
        None => panic!("Cycle Error"),
    }
    mem.data[*curr_mem_add as usize] = token as u8;
    *curr_mem_add += 1;
    let h_byte: u8 = convert_hex_string_to_u8(&value[0..2]);
    let l_byte: u8 = convert_hex_string_to_u8(&value[2..4]);
    mem.data[*curr_mem_add as usize] = l_byte;
    *curr_mem_add += 1;
    mem.data[*curr_mem_add as usize] = h_byte;
    *curr_mem_add += 1;
}

/// Loads an immediate value into memory based on the provided token and value.
///
/// This function stores a byte value corresponding to the provided `token` at the current memory address
/// in the `Memory` structure, then increments the current memory address. It also stores the immediate `value`
/// after converting it, depending on whether the value is in hexadecimal or string format.
///
/// - If the `value` is a valid hex string (starting with `0x`), it is converted and stored as a `u8` byte.
/// - Otherwise, the `value` is interpreted as a regular string and converted to a `u8` using a custom conversion function.
///
/// # Parameters
/// - `token`: A `Token` representing an operation (such as `LDA`, `ADC`, etc.). The token is cast to a `u8`
///   value and stored in the current memory location.
/// - `value`: A string representing the immediate value to be loaded into memory. If the value starts with `0x`,
///   it is treated as a hexadecimal string, otherwise, it is treated as a regular string.
/// - `mem`: A mutable reference to the `Memory` structure where the values are written to.
/// - `curr_mem_add`: A mutable reference to the current memory address. It is updated after each operation.
///
/// # Example
/// ```rust
/// let mut mem = Memory::new();
/// let mut curr_mem_add = 0x1000u16;
/// load_immediate_value(Token::LDA, "0xFF", &mut mem, &mut curr_mem_add);
/// ```
/// This will store the byte corresponding to the `LDA` token in `mem.data[0x1000]`, and the value `0xFF`
/// in `mem.data[0x1001]` after converting it from hexadecimal.
///
/// # Notes
/// - The `value` string must either be in hexadecimal (starting with `0x`) or a regular string. The function handles both cases.
/// - If the value is in hexadecimal, it is expected to be prefixed by `0x` (e.g., `0xFF`).
fn load_immediate_value(token: Token, value: &str, mem: &mut Memory, curr_mem_add: &mut u16,token_cycle_table : &HashMap<Token,u8>) {
    match token_cycle_table.get(&token) {
        Some(num) => mem.data_cycle_count += *num as u32,
        None => panic!("Cycle Error"),
    }
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

/// Loads a relative value into memory based on the provided token.
///
/// This function stores the byte corresponding to the provided `token` at the current memory address
/// in the `Memory` structure and increments the current memory address.
///
/// # Parameters
/// - `token`: A `Token` representing an operation (such as `BEQ`, `BNE`, etc.). The token is cast to a `u8`
///   value and stored in the current memory location.
/// - `mem`: A mutable reference to the `Memory` structure where the token is written to.
/// - `curr_mem_add`: A mutable reference to the current memory address. It is updated after the operation.
///
/// # Example
/// ```rust
/// let mut mem = Memory::new();
/// let mut curr_mem_add = 0x1000u16;
/// load_relative_value(Token::BEQ, &mut mem, &mut curr_mem_add);
/// ```
/// This will store the byte corresponding to the `BEQ` token in `mem.data[0x1000]`, and increment the
/// current memory address to `0x1001`.
///
/// # Notes
/// - This function is typically used for operations that involve relative addressing (like branch instructions),
///   where only the token is stored and the actual relative value will be added in a later step.
fn load_relative_value(token: Token, mem: &mut Memory, curr_mem_add: &mut u16,token_cycle_table : &HashMap<Token,u8>) {
    match token_cycle_table.get(&token) {
        Some(num) => mem.data_cycle_count += *num as u32,
        None => panic!("Cycle Error"),
    }
    mem.data[*curr_mem_add as usize] = token as u8;
    *curr_mem_add += 1;
}

/// Checks if the provided string represents a hexadecimal value.
///
/// This function checks whether the first character of the string is a dollar sign (`$`),
/// which is a common prefix used to denote hexadecimal numbers in some assembly languages.
///
/// # Parameters
/// - `value`: A string reference to the value that needs to be checked for hexadecimal format.
///
/// # Returns
/// - `true`: If the string starts with a `$`, indicating it is a hexadecimal value.
/// - `false`: If the string does not start with a `$`, indicating it is not considered hexadecimal.
///
/// # Panics
/// - This function will panic with a `"Syntax error for hex"` message if the input string is empty.
///
/// # Example
/// ```rust
/// let hex_value = "$FF";
/// let non_hex_value = "FF";
///
/// assert!(is_hex(hex_value));  // returns true
/// assert!(!is_hex(non_hex_value));  // returns false
/// ```
///
/// # Notes
/// - This function is used to identify whether a string represents a hexadecimal value based on the `$` prefix.
/// - It does not check if the rest of the string is a valid hexadecimal number; it only checks the prefix.
fn is_hex(value: &str) -> bool {
    match value.chars().nth(0) {
        Some(c) if c == '$' => {
            return true;
        }
        Some(_) => return false,
        None => panic!("Syntax error for hex"),
    }
}
