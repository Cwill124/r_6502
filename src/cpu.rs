use crate::asm_runner;
use crate::memory::{self, Memory};
use crate::util::check_7_bit;

pub struct CPU {
    pub pc: u16,
    pub sp: u16,

    pub memory: Memory,

    pub x: u8,
    pub a: u8,
    pub y: u8,

    pub c: u8, // Carry Flag
    pub z: u8, // Zero Flag
    pub i: u8, // Interrupt Disable
    pub d: u8, // Decimal Mode Flag
    pub b: u8, // Break Command
    pub v: u8, // Overflow Flag
    pub n: u8, // Negative Flag
}

impl CPU {
    pub fn new() -> Self {
        let mut cpu = CPU {
            pc: 0x0,
            sp: 0x0,
            memory: memory::Memory::new(),
            c: 0,
            z: 0,
            i: 0,
            d: 0,
            b: 0,
            v: 0,
            n: 0,
            x: 0,
            a: 0,
            y: 0,
        };
        cpu.memory.init();
        cpu
    }

    pub fn execute_memory(&mut self) {
        asm_runner::run_memory(self)
    }
    pub fn check_n_flag(&mut self, register: u8) {
        match register {
            0 => {
                if check_7_bit(self.a) {
                    self.n = 1
                } else {
                    self.n = 0;
                }
            }
            1 => {
                if check_7_bit(self.a) {
                    self.n = 1
                } else {
                    self.n = 0;
                }
            }
            2 => {
                if check_7_bit(self.a) {
                    self.n = 1
                } else {
                    self.n = 0;
                }
            }
            _ => println!("Error unknown register"),
        }
    }
    pub fn check_z_flag(&mut self, register: u8) {
        match register {
            0 => {
                if self.a == 0 {
                    self.z = 1
                } else {
                    self.z = 0;
                }
            }
            1 => {
                if self.x == 0 {
                    self.z = 1
                } else {
                    self.z = 0;
                }
            }
            2 => {
                if self.y == 0 {
                    self.z = 1
                } else {
                    self.z = 0;
                }
            }
            _ => println!("Error unknown register"),
        }
    }
}
