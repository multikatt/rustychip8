use std::{fs, io::Error};

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

    pub fn load_rom(&mut self, file_name: &str) -> Result<(), Error> {
        // TODO: Start reading at 0x200
        self.memory = fs::read(file_name)?;
        // for l in &self.memory {
        //     println!("{:#02x}", l);
        // }
        Ok(())
    }

    pub fn fetch(&mut self) -> Result<u16, ()> {
        // let mut first = self.memory[self.pc as usize] as u16;
        let fist = self.memory.get(self.pc as usize);
        let mut first;
        match fist {
            Some(x) => first = *x as u16,
            None => return Err(()),
        }

        // match self.memory[self.pc as usize] as u16 {
        //     3 => {}
        //     Err(err) => println!("{}", err),
        // }
        self.pc += 1;
        let second = self.memory[self.pc as usize] as u16;
        first = first << 8;
        self.pc += 1;
        print!("{:#06x} {:#06x}:  ", self.pc, first + second);
        Ok(first + second)
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

    pub fn decode(&mut self) {
        let next = self.fetch().unwrap();
        let cat: u8 = (next >> 12).try_into().unwrap();
        match cat {
            0x0 => match next {
                0x00e0 => {
                    println!("clear screen");
                }
                _ => println!("{:#04x} instruction not found.", next),
            },
            0x1 => {
                println!("Jump to");
            }
            0x6 => {
                println!("Set register");
            }
            0x7 => {
                println!("Add to register");
            }
            0xa => {
                println!("Set index register");
            }
            _ => println!("{:#04x} {:#04x} not implemented!", cat, next),
        }
    }
}
