use std::env;
use std::process;

mod emulator;
use crate::emulator::Emulator;

fn main() {
    let mut debug_mode = false;
    let mut path = None;
    let mut args = env::args();
    args.next();

    for arg in args {
        match &arg[..] {
            "--debug" | "-d" => debug_mode = true,
            _ => path = Some(arg),
        }
    }

    if let Some(path) = path {
        let mut emulator = Emulator::new(debug_mode);

        if let Err(e) = emulator.load_bin(&path) {
            eprintln!("Chip8 error: {e}");
            process::exit(1);
        }

        emulator.run();
    }
}
