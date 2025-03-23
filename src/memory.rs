const MAX_MEMORY: usize = 65536;

pub struct Memory {
    pub max_memory: usize,
    pub data: [u8; MAX_MEMORY],
    pub data_cycle_count : u32,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            max_memory: MAX_MEMORY,
            data: [0; self::MAX_MEMORY],
            data_cycle_count : 0,
        }
    }

    pub fn init(&mut self) {
        for i in 0..self::MAX_MEMORY {
            self.data[i] = 0;
        }
    }
}
