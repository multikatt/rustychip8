use rand::Rng;
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
    pub key_pressed: Option<u8>,
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
        let stack = vec![];
        let registers = vec![0; 16];
        let display = vec![false; 2048];
        let index = 0x0;
        let pc = 0x200;
        let sp = 0x0;
        let key_pressed = None;
        Self { width, height, memory, stack, registers, display, index, pc, sp, key_pressed }
    }

    pub fn load_rom(&mut self, file_name: &str) -> Result<(), Error> {
        let mut tmp = fs::read(file_name)?;
        let mut buf = vec![0; 0x200];
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
        print!("{:#06x} ", self.pc);
        self.pc += 1;
        let second = self.memory[self.pc as usize] as u16;
        first <<= 8;
        print!("{:#06x}:  ", first + second);
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

    fn set_sprite(&mut self, sprite_height: u16, xcoord: u16, ycoord: u16) {
        for y in 0..sprite_height {
            let mut sprite_data = self.memory[(self.index + y) as usize];
            for x in 0..8 {
                if sprite_data & 0b10000000 != 0 {
                    // println!(
                    //     "y:{} x:{} xcoord:{} ycoord:{} / abs x {} y {}",
                    //     y,
                    //     x,
                    //     xcoord,
                    //     ycoord,
                    //     (xcoord + x) % 64,
                    //     (ycoord + y) % 32
                    // );
                    let pos = self.get_pixel_from_xy((xcoord + x) % 64, (ycoord + y) % 32) as usize;
                    self.display[pos] = true;
                }
                sprite_data <<= 1;
            }
        }
    }

    fn clear_display(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = self.get_pixel_from_xy(x, y) as usize;
                self.display[pos] = false;
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
                    self.clear_display();
                }
                0x00ee => {
                    println!("Return from subroutine");
                    self.pc = self.stack.pop().unwrap();
                }
                _ => println!("{:#06x} instruction not found.", next),
            },
            0x1 => {
                self.pc = next & 0x0fff;
            }
            0x2 => {
                self.stack.push(self.pc);
                self.pc = next & 0x0fff;
            }
            0x3 => {
                let reg = (next & 0x0f00) >> 8;
                let cmp = (next & 0x00ff) as u8;
                println!("SE {:#06x} {:#06x}", reg, cmp);
                if self.registers[reg as usize] == cmp {
                    self.pc += 2;
                }
            }
            0x4 => {
                let reg = (next & 0x0f00) >> 8;
                let cmp = (next & 0x00ff) as u8;
                println!("SNE {:#06x} {:#06x}", reg, cmp);
                if self.registers[reg as usize] != cmp {
                    self.pc += 2;
                }
            }
            0x5 => {
                let cmpx = (next & 0x0f00) >> 8;
                let cmpy = (next & 0x00f0) >> 4;
                println!(
                    "SE {:#06x} {:#06x}",
                    self.registers[cmpx as usize], self.registers[cmpy as usize]
                );
                if self.registers[cmpx as usize] == self.registers[cmpy as usize] {
                    self.pc += 2;
                }
            }
            0x9 => {
                let cmpx = (next & 0x0f00) >> 8;
                let cmpy = (next & 0x00f0) >> 4;
                println!(
                    "SNE {:#06x} {:#06x}",
                    self.registers[cmpx as usize], self.registers[cmpy as usize]
                );
                if self.registers[cmpx as usize] != self.registers[cmpy as usize] {
                    self.pc += 2;
                }
            }
            0x6 => {
                let reg = (next & 0x0f00) >> 8;
                println!("Set register {:#06x} to {:#06x}", reg, next & 0x00ff);
                self.registers[reg as usize] = (next & 0x00ff) as u8;
            }
            0x7 => {
                let reg = (next & 0x0f00) >> 8;
                println!("Adding {:#06x} to register {:#06x}", next & 0x00ff, reg);

                self.registers[reg as usize] =
                    self.registers[reg as usize].overflowing_add((next & 0x00ff) as u8).0;
            }
            0x8 => {
                let vx = (next & 0x0f00) >> 8;
                let vy = (next & 0x00f0) >> 4;
                let cmd = next & 0x000f;

                match cmd {
                    0 => self.registers[vx as usize] = self.registers[vy as usize],
                    1 => self.registers[vx as usize] |= self.registers[vy as usize],
                    2 => self.registers[vx as usize] &= self.registers[vy as usize],
                    3 => self.registers[vx as usize] ^= self.registers[vy as usize],
                    4 => {
                        let res = self.registers[vx as usize]
                            .overflowing_add(self.registers[vy as usize]);
                        self.registers[vx as usize] = res.0;
                        if res.1 {
                            self.registers[0xf] = 1;
                        } else {
                            self.registers[0xf] = 0;
                        }
                    }
                    n @ (5 | 7) => {
                        let res;
                        let mut set_flag = false;

                        match n {
                            5 => {
                                res = self.registers[vx as usize]
                                    .overflowing_sub(self.registers[vy as usize]);
                                if self.registers[vx as usize] > self.registers[vy as usize] {
                                    set_flag = true
                                }
                            }
                            7 => {
                                res = self.registers[vy as usize]
                                    .overflowing_sub(self.registers[vx as usize]);
                                if self.registers[vx as usize] < self.registers[vy as usize] {
                                    set_flag = true
                                }
                            }
                            _ => {
                                panic!("Trying to subtract empty registers");
                            }
                        }
                        self.registers[vx as usize] = res.0;
                        if set_flag {
                            self.registers[0xf] = 1;
                        } else {
                            self.registers[0xf] = 0;
                        }
                    }
                    n @ (0x6 | 0xE) => {
                        // TODO: Set VF
                        let mut vxval = self.registers[((next & 0x0f00) >> 8) as usize];
                        match n {
                            0x6 => vxval >>= 4,
                            0xE => vxval <<= 4,
                            _ => panic!("Trying to shift empty register"),
                        };
                        self.registers[((next & 0x0f00) >> 8) as usize] = vxval;
                    }

                    _ => println!("not implemented"),
                }
            }
            0xa => {
                println!("Set index register to {:#06x}", next & 0x0fff);
                self.index = next & 0x0fff;
            }
            0xc => {
                let vx = (next & 0x0f00) >> 8;
                let nn = (next & 0x00ff) as u8;
                let mut rng = rand::thread_rng();
                let randval: u8 = rng.gen();
                self.registers[vx as usize] = randval & nn;
            }
            0xd => {
                let sprite_height = next & 0x000f;
                let vx = (next & 0x0f00) >> 8;
                let vy = (next & 0x00f0) >> 4;
                let xcoord = self.registers[vx as usize] as u16;
                let ycoord = self.registers[vy as usize] as u16;

                println!("Draw an {} pixels tall sprite from the memory location that the I index register is holding ({:#06x}) to the screen, at the horizontal X coordinate {} in VX ({:#06x}) and the Y {} coordinate in VY ({:#06x})", sprite_height, self.index, xcoord, vx, ycoord, vy );

                self.set_sprite(sprite_height, xcoord, ycoord);
            }
            0xe => {
                let cmd = next & 0x00ff;
                let vx = (next & 0x0f00) >> 8;
                let key = self.registers[vx as usize];
                if self.key_pressed.is_some() {
                    match cmd {
                        0x9E => {
                            if key == self.key_pressed.unwrap() {
                                self.pc += 0x02
                            }
                        }
                        0xA1 => {
                            if key != self.key_pressed.unwrap() {
                                self.pc += 0x02
                            }
                        }
                        _ => panic!("Unknown instruction"),
                    }
                }
            }
            _ => println!("{:#06x} {:#06x} not implemented!", cat, next),
        }
        Ok(())
    }
}

#[test]
fn test_0x00e0() {
    let mut c8 = Chip8::new();
    c8.memory[0x200] = 0x00;
    c8.memory[0x201] = 0xe0;
    c8.display[10] = true;
    c8.decode().unwrap();
    assert_eq!(c8.display[10], false);
}

#[test]
fn test_0x1() {
    let mut c8 = Chip8::new();
    c8.memory[0x200] = 0x11;
    c8.memory[0x201] = 0x11;
    assert_eq!(c8.pc, 0x200);
    c8.decode().unwrap();
    assert_eq!(c8.pc, 0x111);
}

#[test]
fn test_0x2() {
    let mut c8 = Chip8::new();
    c8.memory[0x200] = 0x21;
    c8.memory[0x201] = 0x11;
    assert_eq!(c8.pc, 0x200);
    c8.decode().unwrap();
    assert_eq!(c8.pc, 0x111);
    assert_eq!(c8.stack[0], 0x202);
}

#[test]
fn test_se_and_sne() {
    let mut c8 = Chip8::new();
    // 0x3
    c8.memory[0x200] = 0x30;
    c8.memory[0x201] = 0x22;

    c8.decode().unwrap();
    assert_eq!(c8.pc, 0x202);

    c8.registers[0x0] = 0x22;
    c8.pc = 0x200;

    c8.decode().unwrap();
    assert_eq!(c8.pc, 0x204);

    c8.pc = 0x200;

    // 0x4
    c8.memory[0x200] = 0x40;
    c8.memory[0x201] = 0x33;

    c8.decode().unwrap();
    assert_eq!(c8.pc, 0x204);

    c8.registers[0x0] = 0x33;
    c8.pc = 0x200;

    c8.decode().unwrap();
    assert_eq!(c8.pc, 0x202);

    // 0x5
    c8.registers[0x1] = 0x33;
    c8.memory[0x200] = 0x50;
    c8.memory[0x201] = 0x10;
    c8.pc = 0x200;

    c8.decode().unwrap();
    assert_eq!(c8.pc, 0x204);

    c8.pc = 0x200;
    c8.registers[0x1] = 0x44;

    c8.decode().unwrap();
    assert_eq!(c8.pc, 0x202);

    // 0x9
    c8.registers[0x1] = 0x55;
    c8.memory[0x200] = 0x90;
    c8.memory[0x201] = 0x10;
    c8.pc = 0x200;

    c8.decode().unwrap();
    assert_eq!(c8.pc, 0x204);

    c8.pc = 0x200;
    c8.registers[0x0] = 0x55;

    c8.decode().unwrap();
    assert_eq!(c8.pc, 0x202);
}

#[test]
fn test_0x7() {
    let mut c8 = Chip8::new();
    c8.registers[0x0] = 0x01;
    c8.memory[0x200] = 0x70;
    c8.memory[0x201] = 0x02;

    c8.decode().unwrap();

    assert_eq!(c8.registers[0x0], 0x03);

    c8.registers[0x0] = 0xFF;
    c8.pc = 0x200;
    c8.decode().unwrap();
    assert_eq!(c8.registers[0x0], 0x01);
}

#[test]
fn test_0x8xy4() {
    let mut c8 = Chip8::new();
    c8.registers[0x0] = 0x05;
    c8.registers[0x1] = 0x02;
    c8.memory[0x200] = 0x80;
    c8.memory[0x201] = 0x14;

    c8.decode().unwrap();

    assert_eq!(c8.registers[0x0], 0x07);
    assert_eq!(c8.registers[0xf], 0x00);

    c8.registers[0x0] = 0xff;
    c8.registers[0x1] = 0x02;

    c8.pc = 0x200;
    c8.decode().unwrap();

    assert_eq!(c8.registers[0x0], 0x01);
    assert_eq!(c8.registers[0xf], 0x01);
}

#[test]
fn test_0x8xy_5_and_7() {
    // 5
    let mut c8 = Chip8::new();
    c8.registers[0x0] = 0x05;
    c8.registers[0x1] = 0x02;
    c8.memory[0x200] = 0x80;
    c8.memory[0x201] = 0x15;

    c8.decode().unwrap();

    assert_eq!(c8.registers[0x0], 0x03);
    assert_eq!(c8.registers[0xf], 0x01);

    c8.registers[0x0] = 0x00;
    c8.registers[0x1] = 0x01;

    c8.pc = 0x200;
    c8.decode().unwrap();

    assert_eq!(c8.registers[0x0], 0xff);
    assert_eq!(c8.registers[0xf], 0x00);

    // 7
    c8.registers[0x0] = 0x03;
    c8.registers[0x1] = 0x05;
    c8.memory[0x200] = 0x80;
    c8.memory[0x201] = 0x17;

    c8.pc = 0x200;
    c8.decode().unwrap();

    assert_eq!(c8.registers[0x0], 0x02);
}

#[test]
fn test_0xe() {
    let mut c8 = Chip8::new();
    // 9E
    c8.registers[0x0] = 0x05;
    c8.key_pressed = Some(0x01);
    c8.memory[0x200] = 0xe0;
    c8.memory[0x201] = 0x9e;

    c8.decode().unwrap();

    assert_eq!(c8.pc, 0x202);

    c8.key_pressed = Some(0x05);
    c8.pc = 0x200;

    c8.decode().unwrap();

    assert_eq!(c8.pc, 0x204);
    // A1
    c8.pc = 0x200;
    c8.registers[0x0] = 0x05;
    c8.key_pressed = Some(0x01);
    c8.memory[0x200] = 0xe0;
    c8.memory[0x201] = 0xa1;

    c8.decode().unwrap();

    assert_eq!(c8.pc, 0x204);

    c8.key_pressed = Some(0x05);
    c8.pc = 0x200;

    c8.decode().unwrap();

    assert_eq!(c8.pc, 0x202);
}

#[test]
fn test_pixel_getters() {
    let c8 = Chip8::new();
    assert_eq!(c8.get_pixel_from_xy(5, 0), 5);
    assert_eq!(c8.get_pixel_from_xy(1, 1), 65);
    assert_eq!(c8.get_xy_from_pixel(65), (1, 1));
    assert_eq!(c8.get_xy_from_pixel(5), (5, 0));
}
