use std::error::Error;
use std::fs;

const MEMORY_SIZE: usize = 4096;
const START_ADDRESS: usize = 0x200;
const FONTSET_START_ADDRESS: usize = 0x50;
const FONTSET_SIZE: usize = 80;
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
    i: u16,
    pc: u16,
    opcode: u16,
    sp: u8,
    delay_timer: u8,
    sound_timer: u8,
    v: [u8; 16],
    stack: [u8; 16],
    pub memory: [u8; MEMORY_SIZE],
    keypad: [u8; 16],
    display_buffer: [u8; 64 * 32],
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let mut memory = [0; MEMORY_SIZE];
        memory[FONTSET_START_ADDRESS..FONTSET_END_ADDRESS].copy_from_slice(&FONTSET[..]);
        Chip8 {
            i: 0,
            pc: START_ADDRESS as u16,
            opcode: 0,
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            v: [0; 16],
            stack: [0; 16],
            memory,
            keypad: [0; 16],
            display_buffer: [0; 64 * 32],
        }
    }

    pub fn load_rom(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        let content: Vec<u8> = fs::read(path)?;
        let end_address = START_ADDRESS + content.len();

        self.memory[START_ADDRESS..end_address].copy_from_slice(&content[..]);

        Ok(())
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
// }
