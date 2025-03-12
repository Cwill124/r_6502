use crate::memory::{self, Memory};

pub struct CPU {
    pub pc: u16,
    pub sp: u16,

    pub memory: Memory,

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
        };
        cpu.memory.initialise();
        cpu
    }
    pub fn fetch_address_value(mut self, cycles: &mut u32) -> u8 {
        let value: u8 = self.memory.data[self.pc as usize];

        self.pc += 1;

        *cycles -= 1;

        return value;
    }
}
