use crate::cycle_map;
use crate::token::Token;
use crate::util::combine_address;
use crate::CPU;
use std::collections::HashMap;

const A: u8 = 0;
const X: u8 = 1;
const Y: u8 = 2;

pub fn run_memory(cpu: &mut CPU) {
    let token_cycle_table = cycle_map::init();

    while cpu.memory.data_cycle_count > 0 {
        let current_value: u8 = cpu.memory.data[cpu.pc as usize];
        cycle_a_pc_inc(cpu);

        match current_value {
            0x89 => load(Token::LDA, cpu),
            0xA2 => load(Token::LDX, cpu),
            0xA0 => load(Token::LDY, cpu),
            0xA5 => load(Token::LdaZP, cpu),
            0xAD => load(Token::LdaAP, cpu),
            _ => panic!("Command not found"),
        }
    }
}

fn load(token: Token, cpu: &mut CPU) {
    match token {
        Token::LDA => load_immediate_value(A, cpu),
        Token::LDX => load_immediate_value(X, cpu),
        Token::LDY => load_immediate_value(Y, cpu),
        Token::LdaZP => load_memory_location(A, cpu, true),
        Token::LdaAP => load_memory_location(A, cpu, false),
        _ => panic!("Seg fault"),
    }
}

fn load_immediate_value(register: u8, cpu: &mut CPU) {
    match register {
        A => {
            cpu.a = cpu.memory.data[cpu.pc as usize];
            cpu.check_n_flag(A);
            cpu.check_n_flag(A);
        }
        X => {
            cpu.x = cpu.memory.data[cpu.pc as usize];
            cpu.check_n_flag(X);
            cpu.check_n_flag(X);
        }
        Y => {
            cpu.y = cpu.memory.data[cpu.pc as usize];
            cpu.check_n_flag(Y);
            cpu.check_n_flag(Y);
        }
        _ => panic!("Invalid register code"),
    }

    cycle_a_pc_inc(cpu);
}
fn load_memory_location(register: u8, cpu: &mut CPU, is_zp: bool) {
    match register {
        A => {
            if is_zp {
                load_zp_location(register, cpu)
            } else {
                load_ap_location(register, cpu)
            }
        }
        X => {
            if is_zp {
                load_zp_location(register, cpu)
            } else {
                load_ap_location(register, cpu)
            }
        }
        Y => {
            if is_zp {
                load_zp_location(register, cpu)
            } else {
                load_ap_location(register, cpu)
            }
        }
        _ => panic!("Invalid register code"),
    }
}

fn load_zp_location(register: u8, cpu: &mut CPU) {
    let mem_loc: u8 = cpu.memory.data[cpu.pc as usize];
    cycle_a_pc_inc(cpu);
    match register {
        A => {
            cpu.a = cpu.memory.data[mem_loc as usize];
            cycle_a_pc_inc(cpu);
            cpu.check_n_flag(A);
        }
        X => {
            cpu.x = cpu.memory.data[mem_loc as usize];
            cycle_a_pc_inc(cpu);
            cpu.check_n_flag(X);
        }
        Y => {
            cpu.y = cpu.memory.data[mem_loc as usize];
            cycle_a_pc_inc(cpu);
            cpu.check_n_flag(Y);
        }
        _ => panic!("Invalid register code"),
    }
}

fn load_ap_location(register: u8, cpu: &mut CPU) {
    let l_byte: u8 = cpu.memory.data[cpu.pc as usize];
    cycle_a_pc_inc(cpu);
    let h_byte: u8 = cpu.memory.data[cpu.pc as usize];
    cycle_a_pc_inc(cpu);
    let c_bytes: u16 = combine_address(l_byte, h_byte);
    match register {
        A => {
            cpu.a = cpu.memory.data[c_bytes as usize];
            cycle_a_pc_inc(cpu);
            cpu.check_n_flag(A);
            cpu.check_z_flag(A);
        }
        X => {
            cpu.x = cpu.memory.data[c_bytes as usize];
            cycle_a_pc_inc(cpu);
            cpu.check_n_flag(X);
            cpu.check_z_flag(X);
        }
        Y => {
            cpu.a = cpu.memory.data[c_bytes as usize];
            cycle_a_pc_inc(cpu);
            cpu.check_n_flag(Y);
            cpu.check_z_flag(Y);
        }
        _ => panic!("Invalid register code"),
    }
    cycle_a_pc_inc(cpu);
}

fn cycle_a_pc_inc(cpu: &mut CPU) {
    cpu.memory.data_cycle_count -= 1;
    cpu.pc += 1;
}
