use rand::Rng;
use std::error::Error;
use std::fs;
use std::io;
use sdl2::Sdl;

extern crate sdl2;
mod display;
mod keyboard;

use crate::display::Display;
use crate::display::DisplayConfig;
use crate::keyboard::Keyboard;
use crate::keyboard::KeyboardConfig;

const MEMORY_SIZE: usize = 4096;
const START_ADDRESS: usize = 0x200;
const FONT_SPRITE_SIZE: usize = 5;
const FONTSET_START_ADDRESS: usize = 0x50;
const FONTSET_SIZE: usize = 80;
const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGTH: usize = 32;
const N_REGISTERS: usize = 16;
const N_STACK_LEVELS: usize = 16;
const FONTSET_END_ADDRESS: usize = FONTSET_START_ADDRESS + FONTSET_SIZE;
const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Chip8 {
    i: usize,
    pc: usize,
    opcode: u16,
    sp: usize,
    dt: u8,
    st: u8,
    v: [u8; N_REGISTERS],
    stack: [u16; N_STACK_LEVELS],
    memory: [u8; MEMORY_SIZE],
    keyboard: Keyboard,
    frame_buffer: [u8; DISPLAY_WIDTH * DISPLAY_HEIGTH],
    display: Display,
    display_update: bool,
    sdl_context: Sdl,
    code: u16,
    x: usize,
    y: usize,
    n: u8,
    nn: u8,
    nnn: u16,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        println!("CHIP8: Init memory and registers");
        let mut memory = [0; MEMORY_SIZE];
        memory[FONTSET_START_ADDRESS..FONTSET_END_ADDRESS].copy_from_slice(&FONTSET[..]);
        
        // Setup SDL
        let sdl_context = sdl2::init().unwrap();
        let mut video_subsystem = sdl_context.video().unwrap();
        
        let display_config = DisplayConfig {
            scale: 10.0,
            width: 640, 
            height: 320,
        };
        
        let keyboard_config = KeyboardConfig{};

        Chip8 {
            i: 0,
            pc: START_ADDRESS,
            opcode: 0,
            sp: 0,
            dt: 0,
            st: 0,
            v: [0; N_REGISTERS],
            stack: [0; N_STACK_LEVELS],
            memory,
            keyboard: Keyboard::new(keyboard_config),
            frame_buffer: [0; DISPLAY_WIDTH * DISPLAY_HEIGTH],
            display: Display::new(display_config, &mut video_subsystem),
            display_update: false,
            sdl_context,
            code: 0,
            x: 0,
            y: 0,
            n: 0,
            nn: 0,
            nnn: 0,
        }
    }

    pub fn load_bin(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        println!("CHIP8: Load the binary to memory");
        let content: Vec<u8> = fs::read(path)?;

        self.store_data_ram(content)?;

        Ok(())
    }

    fn store_data_ram(&mut self, data: Vec<u8>) -> Result<(), &'static str> {
        let end_address = START_ADDRESS + data.len();
        if end_address >= MEMORY_SIZE {
            return Err("Data too large to store in the RAM!");
        }
        self.memory[START_ADDRESS..end_address].copy_from_slice(&data[..]);
        Ok(())
    }

    pub fn show_frame_buffer(&self) {
        let line = "-".to_string().repeat(DISPLAY_WIDTH * 2);
        println!("{line}");
        for i in 0..DISPLAY_HEIGTH {
            print!("|{}", self.frame_buffer[i * DISPLAY_WIDTH]);
            for j in 1..DISPLAY_WIDTH {
                print!(" {}", self.frame_buffer[i * DISPLAY_WIDTH + j]);
            }
            println!("|");
        }
        println!("{line}");
    }

    fn fetch(&mut self) {
        println!("CHIP8: Fetch opcode from memory");
        self.opcode = ((self.memory[self.pc] as u16) << 8) | self.memory[self.pc + 1] as u16;
        self.pc += 2;
    }

    fn decode(&mut self) {
        println!("CHIP8: Decode opcode: {:#04X}", self.opcode);
        self.code = self.opcode & 0xf000;
        self.x = (self.opcode & 0x0f00 >> 8) as usize;
        self.y = (self.opcode & 0x00f0 >> 4) as usize;
        self.n = (self.opcode & 0x000f) as u8;
        self.nn = (self.opcode & 0x00ff) as u8;
        self.nnn = self.opcode & 0x0fff;
    }

    fn execute(&mut self) {
        println!("CHIP8: Execute instruction");
        self.display_update = false;

        match self.code {
            // CLS
            0x00e0 => {
                self.frame_buffer = [0; DISPLAY_WIDTH * DISPLAY_HEIGTH];
                self.display_update = true;
            }
            // RET
            0x00ee => {
                self.pc = self.stack[self.sp] as usize;
                self.sp -= 1;
            }
            // JMP addr
            0x1000 => self.pc = self.nnn as usize,
            // CALL addr
            0x2000 => {
                self.sp += 1;
                self.stack[self.sp] = self.pc as u16;
                self.pc = self.nnn as usize;
            }
            // SE Vx, byte
            0x3000 => {
                if self.v[self.x] == self.nn {
                    self.pc += 2;
                }
            }
            // SNE Vx, byte
            0x4000 => {
                if self.v[self.x] != self.nn {
                    self.pc += 2;
                }
            }
            // SE Vx, Vy
            0x5000 => {
                if self.v[self.x] == self.v[self.y] {
                    self.pc += 2;
                }
            }
            // LD Vx, byte
            0x6000 => self.v[self.x] = self.nn,
            // ADD Vx, byte
            0x7000 => {
                let result = self.v[self.x] as u16 + self.nn as u16;
                self.v[self.x] = (result & 0x00ff) as u8;
            }
            0x8000 => match self.n {
                // LD Vx, Vy
                0x0000 => self.v[self.x] = self.v[self.y],
                // OR Vx, Vy
                0x0001 => self.v[self.x] |= self.v[self.y],
                // AND Vx, Vy
                0x0002 => self.v[self.x] &= self.v[self.y],
                // XOR Vx, Vy
                0x0003 => self.v[self.x] ^= self.v[self.y],
                // ADD Vx, Vy
                0x0004 => {
                    let result = self.v[self.x] as u16 + self.v[self.y] as u16;
                    self.v[self.x] = (result & 0x00ff) as u8;
                    // carry
                    self.v[0x0f] = if result > 255 { 1 } else { 0 };
                }
                // SUB Vx, Vy
                0x0005 => {
                    self.v[self.x] = self.v[self.x] - self.v[self.y];
                    // NOT borrow
                    self.v[0x0f] = if self.v[self.x] > self.v[self.y] {
                        1
                    } else {
                        0
                    };
                }
                // SHR Vx {, Vy}
                0x0006 => {
                    self.v[0x0f] = self.v[self.x] & 0x01;
                    self.v[self.x] >>= 1;
                }
                // SUBN Vx, Vy
                0x0007 => {
                    self.v[self.x] = self.v[self.y] - self.v[self.x];
                    // NOT borrow
                    self.v[0x0f] = if self.v[self.y] > self.v[self.x] {
                        1
                    } else {
                        0
                    };
                }
                // SHL Vx {, Vy}
                0x000e => {
                    self.v[0x0f] = self.v[self.x] & 0x80;
                    self.v[self.x] <<= 1;
                }
                _ => {}
            },
            // SNE Vx, Vy
            0x9000 => {
                if self.v[self.x] != self.v[self.y] {
                    self.pc += 2;
                }
            }
            // LD I, addr
            0xa000 => self.i = self.nnn as usize,
            // JP V0, addr
            0xb000 => self.pc = (self.v[0] as u16 + self.nnn) as usize,
            // RND Vx, byte
            0xc000 => self.v[self.x] = rand::thread_rng().gen_range(0..=255) & self.nn,
            // DRW Vx, Vy, nibble
            0xd000 => {
                let x0 = self.v[self.x];
                let y0 = self.v[self.y];

                for y in 0..self.n {
                    let sprite_byte = self.memory[self.i + y as usize];
                    for x in 0..8 {
                        // Get each bit from the sprite byte
                        let pixel = (sprite_byte & (0x80 >> x)) >> (7 - x);
                        let coord_x = (x0 + x) % DISPLAY_WIDTH as u8;
                        let coord_y = (y0 + y) % DISPLAY_HEIGTH as u8;
                        let coordinate =
                            (coord_x + (coord_y as usize * DISPLAY_WIDTH) as u8) as usize;
                        // Check collision
                        self.v[0x0f] = if self.frame_buffer[coordinate] == 1 && pixel == 1 {
                            1
                        } else {
                            0
                        };
                        self.frame_buffer[coordinate] ^= pixel;
                    }
                }
                self.display_update = true;
            }
            0xe000 => match self.nn {
                // SKP Vx
                0x009e => {
                    if self.keyboard.is_key_pressed(self.v[self.x]) {
                        self.pc += 2;
                    }
                }
                // SKNP Vx
                0x00a1 => {
                    if self.keyboard.is_key_not_pressed(self.v[self.x]) {
                        self.pc += 2;
                    }
                }
                _ => {}
            },
            0xf000 => match self.nn {
                // LD Vx, DT
                0x0007 => self.v[self.x] = self.dt,
                // LD Vx, K
                0x000a => {
                    let mut event_pump = self.sdl_context.event_pump().unwrap();
                    self.v[self.x] = 'polling: loop {
                        self.keyboard.state_update(&mut event_pump);
                        if let Some(key) = self.keyboard.get_pressed_key() {
                            break 'polling key;
                        }
                    };
                }
                // LD DT, Vx
                0x0015 => self.dt = self.v[self.x],
                // LD ST, Vx
                0x0018 => self.st = self.v[self.x],
                // ADD I, Vx
                0x001e => self.i += self.v[self.x] as usize,
                // LD F, Vx
                0x0029 => {
                    self.i = (self.v[self.x] as usize * FONT_SPRITE_SIZE) + FONTSET_START_ADDRESS
                }
                // LD B, Vx
                0x0033 => {
                    self.memory[self.i] = self.v[self.x] / 100;
                    self.memory[self.i + 1] = (self.v[self.x] % 100) / 10;
                    self.memory[self.i + 2] = self.v[self.x] % 10;
                }
                // LD [I], Vx
                0x0055 => {
                    for i in 0..N_REGISTERS {
                        self.memory[self.i + i] = self.v[i];
                    }
                }
                // LD Vx, [I]
                0x0065 => {
                    for i in 0..N_REGISTERS {
                        self.v[i] = self.memory[self.i + 1];
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn step(&mut self) {
        self.fetch();
        self.decode();
        self.execute();
    }

    pub fn run(&mut self) {
        let mut step_by_step = true;
        let mut event_pump = self.sdl_context.event_pump().unwrap();

        'running: loop {
            // Run instruction
            self.step();

            if step_by_step {
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let input = input.trim();
                match input {
                    "q" | "quit" => break 'running,
                    "d" => self.show_frame_buffer(),
                    "run" => step_by_step = false,
                    _ => {}
                }
            }

            // Update display
            if self.display_update {
                self.display.draw(self.frame_buffer.to_vec());
            }

            // Update keyboard state
            self.keyboard.state_update(&mut event_pump);

            // Check quit event
            if self.display.check_quit_event(&mut event_pump) {
                break 'running;
            }
        }
    }
}