extern crate sdl2;
use chip8_rs::Chip8;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use sdl2::Sdl;
use std::error::Error;
use std::fs;
use std::io;
use std::time::Duration;

pub struct UI {
    sdl_context: Sdl,
    canvas: WindowCanvas,
    chip8: Chip8,
}

impl UI {
    pub fn new() -> UI {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("CHIP8", 640, 320)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_scale(10.0, 10.0).unwrap();
        let chip8 = Chip8::new();

        UI {
            sdl_context,
            canvas,
            chip8,
        }
    }

    pub fn screen_update(&mut self) {
        let frame_buffer = self.chip8.get_frame_buffer();

        self.canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
        self.canvas.clear();
        self.canvas.set_draw_color(Color::RGBA(255, 255, 255, 255));
        for i in 0..32 {
            for j in 0..64 {
                if frame_buffer[i * 64 + j] == 1 {
                    self.canvas
                        .draw_point(Point::new(j as i32, i as i32))
                        .unwrap();
                }
            }
        }
        self.canvas.present();
    }

    pub fn check_quit_event(&mut self) -> bool {
        let mut event_pump = self.sdl_context.event_pump().unwrap();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return true,
                _ => {}
            }
        }
        false
    }

    pub fn keyboard_update(&mut self) {
        self.sdl_context
            .event_pump()
            .unwrap()
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .for_each(|key| {
                match key {
                    Keycode::Num1 => self.chip8.keypress(0x01),
                    Keycode::Num2 => self.chip8.keypress(0x02),
                    Keycode::Num3 => self.chip8.keypress(0x03),
                    Keycode::Num4 => self.chip8.keypress(0x0c),
                    Keycode::Q => self.chip8.keypress(0x04),
                    Keycode::W => self.chip8.keypress(0x05),
                    Keycode::E => self.chip8.keypress(0x06),
                    Keycode::R => self.chip8.keypress(0x0d),
                    Keycode::A => self.chip8.keypress(0x07),
                    Keycode::S => self.chip8.keypress(0x08),
                    Keycode::D => self.chip8.keypress(0x09),
                    Keycode::F => self.chip8.keypress(0x0e),
                    Keycode::Z => self.chip8.keypress(0x0a),
                    Keycode::X => self.chip8.keypress(0x00),
                    Keycode::C => self.chip8.keypress(0x0b),
                    Keycode::V => self.chip8.keypress(0x0f),
                    _ => {}
                };
            });
    }

    pub fn load_bin(&mut self, path: &str) -> Result<(), Box<dyn Error>> {
        let content: Vec<u8> = fs::read(path)?;

        self.chip8.store_data_ram(content)?;

        Ok(())
    }

    pub fn run(&mut self) {
        let mut step_by_step = true;

        'running: loop {
            // Update keyboard state
            self.keyboard_update();

            // Check quit event
            if self.check_quit_event() {
                break 'running;
            }

            // Run instruction
            self.chip8.tick();

            // Update screen
            if self.chip8.draw {
                self.screen_update();
            }

            if step_by_step {
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let input = input.trim();
                match input {
                    "q" | "quit" => break 'running,
                    "d" => self.chip8.show_frame_buffer(),
                    "r" | "run" => step_by_step = false,
                    _ => {}
                }
            }

            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}
