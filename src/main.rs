use asm_parser::read_asm_file;
use cpu::CPU;

mod asm_parser;
mod cpu;
mod memory;
mod token;
mod util;
fn print_memory_table(memory: &[u8]) {
    let mut stop: bool = false;
    let mut i: usize = 0;

    println!("#### MEMORY TABLE #####");

    while !stop {
        let value = memory[i];
        if i < 100 {
            let hex_string = format!("{:04X}", i);
            let hex_value = format!("{:02X}", value);
            let bin = format!("{:08b}", value);
            println!(
                "Memory location: 0x{}, value: {}, hex: 0x{}, bin: {}",
                hex_string, value, hex_value, bin
            );
            i += 1;
        } else {
            stop = true;
        }
    }
}

fn main() {
    let mut cpu = CPU::new();
    let mut starting_add: u16 = 0;
    read_asm_file("test.asm".to_string(), &mut cpu.memory, &mut starting_add);
    print_memory_table(&cpu.memory.data);
}
