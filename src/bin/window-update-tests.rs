#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
// TODO: Remove above
#![allow(non_snake_case)]

use std::thread;
use std::time::Duration;

use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn sleep(secs: u64) {
    // sleep the current thread for `secs` seconds
    thread::sleep(Duration::from_secs(secs));
}

fn main() {
    let mut buffer: Vec<u32> = vec![0x00FF0000; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap();

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);


    println!("log1");
    window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    println!("log2");
    println!("Press a now...");
    sleep(10);
    println!("log3");
    window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    let keysPressed = window.get_keys_pressed(minifb::KeyRepeat::No);
    if !keysPressed.is_empty() {
        println!("keysPressed = {:?}", keysPressed);
    }
    println!("log4");
    let keysReleased = window.get_keys_released();
    if !keysReleased.is_empty() {
        println!("keysReleased = {:?}", keysReleased);
    }
    println!("log5");





    // println!("log1");
    // sleep(10); // I press the 'a' key here, to be clear
    // println!("log1.1");
    // sleep(10);
    // println!("log1.2");
    // let keysPressed = window.get_keys_pressed(minifb::KeyRepeat::No);
    // if !keysPressed.is_empty() {
    //     // eprintln!("keysPressed = {:?}", keysPressed);
    //     println!("keysPressed = {:?}", keysPressed);
    // }
    // println!("log2");
    // let keysReleased = window.get_keys_released();
    // if !keysReleased.is_empty() {
    //     println!("keysReleased = {:?}", keysReleased);
    // }
    // println!("log3");


}
