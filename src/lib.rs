use rand::Rng;
use std::error::Error;
use std::fs;
use std::io;

extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;

const MEMORY_SIZE: usize = 4096;
const START_ADDRESS: usize = 0x200;
const FONT_SPRITE_SIZE: usize = 5;
const FONTSET_START_ADDRESS: usize = 0x50;
const FONTSET_SIZE: usize = 80;
const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGTH: usize = 32;
const KEY_PRESSED: u8 = 1;
const KEY_NOT_PRESSED: u8 = 0;
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
    v: [u8; 16],
    stack: [u16; 16],
    pub memory: [u8; MEMORY_SIZE],
    keyboard: [u8; 16],
    display_buffer: [u8; DISPLAY_WIDTH * DISPLAY_HEIGTH],
    display_update: bool,
    code: u16,
    x: usize,
    y: usize,
    n: u8,
    kk: u8,
    nnn: u16,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        println!("CHIP8: Init memory and registers");
        let mut memory = [0; MEMORY_SIZE];
        memory[FONTSET_START_ADDRESS..FONTSET_END_ADDRESS].copy_from_slice(&FONTSET[..]);
        Chip8 {
            i: 0,
            pc: START_ADDRESS,
            opcode: 0,
            sp: 0,
            dt: 0,
            st: 0,
            v: [0; 16],
            stack: [0; 16],
            memory,
            keyboard: [0; 16],
            display_buffer: [0; 64 * 32],
            display_update: false,
            code: 0,
            x: 0,
            y: 0,
            n: 0,
            kk: 0,
            nnn: 0,
        }
    }

    pub fn load_rom(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        println!("CHIP8: Load ROM to memory");
        let content: Vec<u8> = fs::read(path)?;
        let end_address = START_ADDRESS + content.len();

        self.memory[START_ADDRESS..end_address].copy_from_slice(&content[..]);

        Ok(())
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
        self.kk = (self.opcode & 0x00ff) as u8;
        self.nnn = self.opcode & 0x0fff;
    }

    fn execute(&mut self) {
        println!("CHIP8: Execute instruction");
        self.display_update = false;

        match self.code {
            // CLS
            0x00e0 => {
                self.display_buffer = [0; DISPLAY_WIDTH * DISPLAY_HEIGTH];
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
                if self.v[self.x] == self.kk {
                    self.pc += 2;
                }
            }
            // SNE Vx, byte
            0x4000 => {
                if self.v[self.x] != self.kk {
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
            0x6000 => self.v[self.x] = self.kk,
            // ADD Vx, byte
            0x7000 => {
                let result = self.v[self.x] as u16 + self.kk as u16;
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
            0xc000 => self.v[self.x] = rand::thread_rng().gen_range(0..=255) & self.kk,
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
                        self.v[0x0f] = if self.display_buffer[coordinate] == 1 && pixel == 1 {
                            1
                        } else {
                            0
                        };
                        self.display_buffer[coordinate] ^= pixel;
                    }
                }
                self.display_update = true;
            }
            0xe000 => match self.kk {
                // SKP Vx
                0x009e => {
                    if self.keyboard[self.v[self.x] as usize] == KEY_PRESSED {
                        self.pc += 2;
                    }
                }
                // SKNP Vx
                0x00a1 => {
                    if self.keyboard[self.v[self.x] as usize] == KEY_NOT_PRESSED {
                        self.pc += 2;
                    }
                }
                _ => {}
            },
            0xf000 => match self.kk {
                // LD Vx, DT
                0x0007 => self.v[self.x] = self.dt,
                // LD Vx, K
                0x000a => {
                    self.v[self.x] = 'outer: loop {
                        // TODO: Add keyboard state update
                        for (i, key) in self.keyboard.iter().enumerate() {
                            if *key == KEY_PRESSED {
                                break 'outer i;
                            }
                        }
                    } as u8;
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
                    for i in 0..16 {
                        self.memory[self.i + i] = self.v[i];
                    }
                }
                // LD Vx, [I]
                0x0065 => {
                    for i in 0..16 {
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
        // Setup SDL
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("CHIP8", 640, 320)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_scale(10.0, 10.0).unwrap();

        let mut event_pump = sdl_context.event_pump().unwrap();

        'running: loop {
            // Run instruction
            self.step();

            // Update display
            if self.display_update {
                canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
                canvas.clear();
                canvas.set_draw_color(Color::RGBA(255, 255, 255, 255));
                for i in 0..32 {
                    for j in 0..64 {
                        if self.display_buffer[i * 64 + j] == 1 {
                            canvas.draw_point(Point::new(j as i32, i as i32)).unwrap();
                        }
                    }
                }
                canvas.present();
            }

            // Update keyboard
            self.keyboard = [0; 16];
            event_pump
                .keyboard_state()
                .pressed_scancodes()
                .filter_map(Keycode::from_scancode)
                .for_each(|key| {
                    match key {
                        Keycode::Num1 => self.keyboard[0x1] = 1,
                        Keycode::Num2 => self.keyboard[0x2] = 1,
                        Keycode::Num3 => self.keyboard[0x3] = 1,
                        Keycode::Num4 => self.keyboard[0xc] = 1,
                        Keycode::Q => self.keyboard[0x4] = 1,
                        Keycode::W => self.keyboard[0x5] = 1,
                        Keycode::E => self.keyboard[0x6] = 1,
                        Keycode::R => self.keyboard[0xd] = 1,
                        Keycode::A => self.keyboard[0x7] = 1,
                        Keycode::S => self.keyboard[0x8] = 1,
                        Keycode::D => self.keyboard[0x9] = 1,
                        Keycode::F => self.keyboard[0xe] = 1,
                        Keycode::Z => self.keyboard[0xa] = 1,
                        Keycode::X => self.keyboard[0x0] = 1,
                        Keycode::C => self.keyboard[0xb] = 1,
                        Keycode::V => self.keyboard[0xf] = 1,
                        _ => {}
                    };
                });

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running,
                    _ => {}
                }
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
// }
