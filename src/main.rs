use chip8_rs::Chip8;
use std::env;
use std::process;

fn main() {
    let mut args = env::args();
    args.next();
    let path = args.next().unwrap_or_else(|| {
        eprintln!("Missing arguments");
        process::exit(1);
    });

    let mut chip8 = Chip8::new();

    if let Err(e) = chip8.load_rom(&path) {
        eprintln!("Chip8 error: {e}");
        process::exit(1);
    }

    println!("memory: {:?}", chip8.memory);
}
