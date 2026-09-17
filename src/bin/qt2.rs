use minifb::{Key, Window, WindowOptions};

fn main() {
    let width = 640;
    let height = 480;
    let mut buffer: Vec<u32> = vec![0; width * height];

    let mut window = Window::new(
        "minifb 0.28.0 Key Release Example",
        width,
        height,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // 1. Pump OS events to populate the key states
        window.update_with_buffer(&buffer, width, height).unwrap();

        // 2. Direct Vec<Key> iteration for minifb 0.28.0
        let released_keys = window.get_keys_released();
        for key in released_keys {
            eprintln!("Key released: {:?}", key);
        }
    }
}