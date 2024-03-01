use std::env;
use std::process;

mod ui;
use crate::ui::UI;

fn main() {
    let mut args = env::args();
    args.next();
    let path = args.next().unwrap_or_else(|| {
        eprintln!("Missing arguments");
        process::exit(1);
    });

    let mut ui = UI::new();

    if let Err(e) = ui.load_bin(&path) {
        eprintln!("Chip8 error: {e}");
        process::exit(1);
    }

    ui.run();
}
