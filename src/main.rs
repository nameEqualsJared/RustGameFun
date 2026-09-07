#![allow(dead_code)]
#![allow(unused_variables)]
// TODO: Remove above
#![allow(non_snake_case)]

mod graphics;
// mod game;

const WINDOW_TITLE: &str = "Test - ESC to exit";
const SCREEN_WIDTH: usize = 640;
const SCREEN_HEIGHT: usize = 360;

fn main() {
    let mut buffer: Vec<u32> = vec![0; SCREEN_WIDTH * SCREEN_HEIGHT];

    let mut graphics = graphics::Graphics::initialize(WINDOW_TITLE, SCREEN_WIDTH, SCREEN_HEIGHT);

    loop {
        for i in buffer.iter_mut() {
            *i = 0x00FF00FF;
        }

        // let keysPressed = window.get_keys_pressed(minifb::KeyRepeat::No);
        // if !keysPressed.is_empty() {
        //     eprintln!("keysPressed = {:?}", keysPressed);
        // }


        graphics.updateScreen(&buffer, SCREEN_WIDTH, SCREEN_HEIGHT);

        // TODO: Implement something like a check for
        //   while window.is_open() && !window.is_key_down(Key::Escape)
        // so you know, I can close the app haha

    }

}
