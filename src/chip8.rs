use std::fs;

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
        let index = 0x0;
        let pc = 0x0;
        let sp = 0x0;
        Self { memory, stack, registers, index, pc, sp }
    }

    pub fn load_rom(&mut self, file_name: &str) {
        // TODO: Start reading at 0x200
        self.memory = fs::read(file_name).expect("Could not read file");
        // for l in &self.memory {
        //     println!("{:#02x}", l);
        // }
    }

    pub fn fetch(&mut self) -> u16 {
        let mut first = self.memory[self.pc as usize] as u16;
        self.pc += 1;
        let second = self.memory[self.pc as usize] as u16;
        first = first << 8;
        self.pc += 1;
        return first + second;
    }

    // pub fn fetch(&self) -> u16 {
    //     let mut one = self.memory[self.pc as usize] as u16;
    //     let two = self.memory[self.pc as usize + 1] as u16;
    //     println!("{:#02x}", one);
    //     one = one << 8;
    //     println!("{:#02x}", one);
    //     let three = one + two;
    //     println!("{:#02x}", two);
    //     println!("{:#04x}", three);
    //     return one;
    // }

    // pub fn decode(&mut self) {
    //     let next = self.fetch();
    //     match next {
    //         Some(expr) => expr,
    //         None => expr,
    //     }
    // }
}
