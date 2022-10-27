pub struct Chip8 {
    memory: Vec<u8>,
    stack: Vec<u16>,
    registers: Vec<u8>,
    index: u16,
    pc: u16,
    sp: u8,
    /*
        delay timer
        sound timer
    */
}

impl Chip8 {
    pub fn new() -> Self {
        let memory = Vec::with_capacity(4096);
        let stack = Vec::with_capacity(16);
        let registers = Vec::with_capacity(16);
        let index = 0;
        let pc = 0;
        let sp = 0;
        Self { memory, stack, registers, index, pc, sp }
    }

    pub fn load_rom(&self, file_name: &str) {}
}
