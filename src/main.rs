use asm_parser::read_asm_file;
use cpu::CPU;

mod asm_parser;
mod cpu;
mod memory;

fn main() {
    let mut cpu = CPU::new();
    read_asm_file("test.asm".to_string(), &mut cpu.memory);
}
