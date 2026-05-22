use crate::game::Game;
use minifb::{Key, Window, WindowOptions};

mod color;
mod element;
mod game;
mod particle;
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
        game.tick();

        game.render(&mut screen_buffer);

        window
            .update_with_buffer(&screen_buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
