use asm_parser::read_asm_file;
use cpu::CPU;

mod asm_parser;
mod cpu;
mod memory;
mod token;
mod util;

fn main() {
    let mut cpu = CPU::new();
    let mut starting_add: u16 = 0;
    read_asm_file("test.asm".to_string(), &mut cpu.memory, &mut starting_add);
    println!("{}", cpu.memory.data[0]);
    println!("{}", cpu.memory.data[1]);
}
