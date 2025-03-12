const MAX_MEMORY : usize = 65536;

pub struct Memory {
    pub max_memory: usize,
    pub data: [u8; MAX_MEMORY], 
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            max_memory: MAX_MEMORY,
            data: [0; self::MAX_MEMORY], 
        }
    }

    pub fn initialise(&mut self) {
        for i in 0..self::MAX_MEMORY {
            self.data[i] = 0;
        }
    }
}
