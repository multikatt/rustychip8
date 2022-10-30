use std::{fs, io::Error};

pub struct Chip8 {
    width: u16,
    height: u16,
    memory: Vec<u8>,
    stack: Vec<u16>,
    registers: Vec<u8>,
    pub display: Vec<bool>,
    index: u16,
    pc: u16,
    sp: u8,
    /*
        delay timer
        sound timer
    */
}

impl Default for Chip8 {
    fn default() -> Self {
        Self::new()
    }
}

impl Chip8 {
    pub fn new() -> Self {
        let width = 64;
        let height = 32;
        let memory = vec![0; 4096];
        let stack = vec![0; 16];
        let registers = vec![0; 16];
        let display = vec![false; 2048];
        let index = 0x0;
        let pc = 0x1ff;
        let sp = 0x0;
        Self { width, height, memory, stack, registers, display, index, pc, sp }
    }

    pub fn load_rom(&mut self, file_name: &str) -> Result<(), Error> {
        let mut tmp = fs::read(file_name)?;
        let mut buf = vec![0; 0x1ff];
        buf.append(&mut tmp);
        self.memory = buf;

        Ok(())
    }

    pub fn fetch(&mut self) -> Result<u16, ()> {
        let check_if_empty = self.memory.get(self.pc as usize); // TODO
        let mut first;
        match check_if_empty {
            Some(x) => first = *x as u16,
            None => return Err(()),
        }

        self.pc += 1;
        let second = self.memory[self.pc as usize] as u16;
        first <<= 8;
        print!("{:#06x} {:#06x}:  ", self.pc, first + second);
        self.pc += 1;
        Ok(first + second)
    }

    pub fn get_pixel_from_xy(&self, xcoord: u16, ycoord: u16) -> u16 {
        ((ycoord * self.width) + xcoord) as u16
    }

    pub fn get_xy_from_pixel(&self, pixel: u16) -> (u8, u8) {
        let x = pixel % (self.width as u16);
        let y = pixel / (self.width as u16);
        (x as u8, y as u8)
    }

    fn set_display(&mut self, sprite_height: u16, xcoord: u16, ycoord: u16) {
        for y in 0..sprite_height {
            let mut sprite_data = self.memory[(self.index + y) as usize];
            for x in 0..8 {
                if sprite_data & 0b10000000 != 0 {
                    println!(
                        "y:{} x:{} xcoord:{} ycoord:{} / abs x {} y {}",
                        y,
                        x,
                        xcoord,
                        ycoord,
                        (xcoord + x) % 64,
                        (ycoord + y) % 32
                    );
                    let pos = self.get_pixel_from_xy((xcoord + x) % 64, (ycoord + y) % 32) as usize;
                    self.display[pos] = true;
                }
                sprite_data <<= 1;
            }
        }
    }

    pub fn decode(&mut self) -> Result<(), ()> {
        let next = self.fetch()?;
        let cat: u8 = (next >> 12).try_into().unwrap();
        match cat {
            0x0 => match next {
                0x00e0 => {
                    println!("clear screen");
                }
                _ => println!("{:#06x} instruction not found.", next),
            },
            0x1 => {
                // println!("Jump to {:#06x}", next & 0x0fff);
                self.pc = next & 0x0fff;
                self.pc -= 1; // Why is this needed?
            }
            0x6 => {
                let reg = (next & 0x0f00) >> 8;
                println!("Set register {:#06x} to {:#06x}", reg, next & 0x00ff);
                self.registers[reg as usize] = (next & 0x00ff) as u8;
            }
            0x7 => {
                let reg = (next & 0x0f00) >> 8;
                println!("Adding {:#06x} to register {:#06x}", next & 0x00ff, reg);
                self.registers[reg as usize] += (next & 0x00ff) as u8;
            }
            0xa => {
                println!("Set index register to {:#06x}", next & 0x0fff);
                self.index = next & 0x0fff;
                self.index -= 1;
            }
            0xd => {
                let sprite_height = next & 0x000f;
                let vx = (next & 0x0f00) >> 8;
                let vy = (next & 0x00f0) >> 4;
                let xcoord = self.registers[vx as usize] as u16;
                let ycoord = self.registers[vy as usize] as u16;

                println!("Draw an {} pixels tall sprite from the memory location that the I index register is holding ({:#06x}) to the screen, at the horizontal X coordinate {} in VX ({:#06x}) and the Y {} coordinate in VY ({:#06x})", sprite_height, self.index, xcoord, vx, ycoord, vy );

                self.set_display(sprite_height, xcoord, ycoord);
            }
            _ => println!("{:#06x} {:#06x} not implemented!", cat, next),
        }
        Ok(())
    }
}
