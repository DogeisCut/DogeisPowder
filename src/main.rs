#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::game::Game;
use minifb::{Key, Window, WindowOptions};

mod color;
mod element;
mod game;
mod particle;
mod shape_utils;
mod tool;
mod world;

const WIDTH: usize = 320;
const HEIGHT: usize = 180;

fn main() {
    let mut game: Game = Game::new(WIDTH, HEIGHT);

    let mut window = Window::new(
        "DogeisPowder",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: minifb::Scale::X4,
            ..WindowOptions::default()
        },
    )
    .expect("Failed to create window :(");

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut screen_buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    while window.is_open() && !window.is_key_pressed(Key::Escape, minifb::KeyRepeat::No) {
        if let Some((mouse_x, mouse_y)) = window.get_mouse_pos(minifb::MouseMode::Clamp) {
            let x = mouse_x as usize;
            let y = mouse_y as usize;

            game.mouse_x = x;
            game.mouse_y = y;
        }

        game.tick();

        game.render(&mut screen_buffer);

        window
            .update_with_buffer(&screen_buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
