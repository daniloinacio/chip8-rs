extern crate sdl2;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

const KEY_PRESSED: u8 = 1;
const KEY_NOT_PRESSED: u8 = 0;

pub struct KeyboardConfig {
}

pub struct Keyboard {
    keyboard_state: Vec<u8>,
}

impl Keyboard {
    pub fn new(config: KeyboardConfig) -> Keyboard {
        Keyboard {
            keyboard_state: vec![0; 16],
        }
    }

    pub fn is_key_pressed(&self, key: u8) -> bool {
        self.keyboard_state[key as usize] == KEY_PRESSED
    }

    pub fn is_key_not_pressed(&self, key: u8) -> bool {
        self.keyboard_state[key as usize] == KEY_NOT_PRESSED
    }

    pub fn get_pressed_key(&self) -> Option<u8> {
        for (key, state) in self.keyboard_state.iter().enumerate() {
            if *state == KEY_PRESSED {
                return Some(key as u8);
            }
        }
        None
    }

    pub fn state_update(&mut self, event_pump: &mut EventPump) {
        self.keyboard_state = vec![KEY_NOT_PRESSED; 16];
        event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .for_each(|key| {
                match key {
                    Keycode::Num1 => self.keyboard_state[0x01] = KEY_PRESSED,
                    Keycode::Num2 => self.keyboard_state[0x02] = KEY_PRESSED,
                    Keycode::Num3 => self.keyboard_state[0x03] = KEY_PRESSED,
                    Keycode::Num4 => self.keyboard_state[0x0c] = KEY_PRESSED,
                    Keycode::Q => self.keyboard_state[0x04] = KEY_PRESSED,
                    Keycode::W => self.keyboard_state[0x05] = KEY_PRESSED,
                    Keycode::E => self.keyboard_state[0x06] = KEY_PRESSED,
                    Keycode::R => self.keyboard_state[0x0d] = KEY_PRESSED,
                    Keycode::A => self.keyboard_state[0x07] = KEY_PRESSED,
                    Keycode::S => self.keyboard_state[0x08] = KEY_PRESSED,
                    Keycode::D => self.keyboard_state[0x09] = KEY_PRESSED,
                    Keycode::F => self.keyboard_state[0x0e] = KEY_PRESSED,
                    Keycode::Z => self.keyboard_state[0x0a] = KEY_PRESSED,
                    Keycode::X => self.keyboard_state[0x00] = KEY_PRESSED,
                    Keycode::C => self.keyboard_state[0x0b] = KEY_PRESSED,
                    Keycode::V => self.keyboard_state[0x0f] = KEY_PRESSED,
                    _ => {}
                };
            });
    }
}
