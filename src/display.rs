extern crate sdl2;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::WindowCanvas;
use sdl2::EventPump;
use sdl2::VideoSubsystem;

pub struct DisplayConfig {
    pub scale: f32,
    pub width: u32,
    pub height: u32,
}

pub struct Display {
    canvas: WindowCanvas
}

impl Display {
    pub fn new(config: DisplayConfig, video_subsystem: &mut VideoSubsystem) -> Display {
        let window = video_subsystem
            .window("CHIP8", config.width, config.height)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();
        canvas.set_scale(config.scale, config.scale).unwrap();
        Display {
            canvas
        }
    }

    pub fn draw(&mut self, frame_buffer: Vec<u8>) {
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

    pub fn check_quit_event(&mut self, event_pump: &mut EventPump) -> bool {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => return true,
                _ => {}
            }
        }
        false
    }
}
