
use minifb::{Window, WindowOptions};

pub struct Graphics {
    window: Window
}

impl Graphics {
    pub fn initialize(window_name: &str, window_width: usize, window_height: usize) -> Self {
        let mut window = Window::new(
            window_name,
            window_width,
            window_height,
            WindowOptions::default(),
        ).unwrap();

        // Limit to max ~60 fps update rate
        window.set_target_fps(60);

        Graphics {
            window
        }
    }

    pub fn updateScreen(&mut self, buffer: &[u32], width: usize, height: usize) {
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        self.window.update_with_buffer(buffer, width, height).unwrap();
    }

}