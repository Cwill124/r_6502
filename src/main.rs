mod memory;
mod cpu;

fn main() {
    let mut memory = memory::Memory::new();
    memory.initialise();
    println!("Memory initialized with size: {}", memory.max_memory);
}
