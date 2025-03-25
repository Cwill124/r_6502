use crate::cycle_map;
use crate::token::Token;
use crate::CPU;
use std::collections::HashMap;

const A: u8 = 0;
const X: u8 = 1;
const Y: u8 = 2;

pub fn run_memory(cpu: &mut CPU) {
    let token_cycle_table = cycle_map::init();

    while cpu.memory.data_cycle_count > 0 {
        let current_value: u8 = cpu.memory.data[cpu.pc as usize];
        cpu.memory.data_cycle_count -= 1;
        cpu.pc += 1;

        match current_value {
            0x89 => lda(Token::LDA, cpu),
            0xA2 => lda(Token::LDX, cpu),
            0xA0 => lda(Token::LDY, cpu),
            _ => panic!("Command not found"),
        }
    }
}

fn lda(token: Token, cpu: &mut CPU) {
    match token {
        Token::LDA => load_immediate_value(A, cpu),
        Token::LDX => load_immediate_value(X, cpu),
        Token::LDY => load_immediate_value(Y, cpu),
        _ => panic!("Seg fault"),
    }
}

fn load_immediate_value(register: u8, cpu: &mut CPU) {
    match register {
        A => {
            cpu.a = cpu.memory.data[cpu.pc as usize];
        }
        X => {
            cpu.x = cpu.memory.data[cpu.pc as usize];
        }
        Y => {
            cpu.y = cpu.memory.data[cpu.pc as usize];
        }
        _ => panic!("Invalid register code"),
    }
    cpu.memory.data_cycle_count -= 1;
    cpu.pc += 1;
}
