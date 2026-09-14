use minifb::{Key, Window, WindowOptions};
use std::io::{self, Write};

fn main() {
    let mut window = Window::new(
        "Diagnostics",
        300, 300,
        WindowOptions::default()
    ).unwrap();

    println!("Diagnostic started. Click this window and tap 'E'.");

    while window.is_open() {
        window.update();

        // Diagnostic 1: Explicit boolean check
        if window.is_key_released(Key::E) {
            eprintln!("[BOOLean] E was released!");
        }

        // Diagnostic 2: Vector check
        let released = window.get_keys_released();
        if !released.is_empty() {
            eprintln!("[VECTOR] Keys released: {:?}", released);
        }
    }
}
