#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::{
    game::Game,
    input::{InputMap, InputState},
};
use minifb::{Key, Window, WindowOptions};

// Please keep this hierarchy clean.
mod color;
mod raster;
mod vector;

mod element;
mod input;

mod particle;
mod shape;

mod tool;
mod world;

mod game;

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

    //window.set_cursor_visibility(false);
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    //window.limit_update_rate(None);

    let mut screen_buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut input_map = InputMap::new();

    input_map.add_action_event(
        input::Action::UseTool,
        input::RawInput::Mouse(minifb::MouseButton::Left),
    );
    input_map.add_action_event(
        input::Action::UseSecondaryTool,
        input::RawInput::Mouse(minifb::MouseButton::Right),
    );
    input_map.add_action_event(
        input::Action::UseTrinaryTool,
        input::RawInput::Mouse(minifb::MouseButton::Middle),
    );
    input_map.add_action_event(
        input::Action::SwitchToolShape,
        input::RawInput::Keyboard(Key::Tab),
    );
    input_map.add_action_event(input::Action::EnlargeToolShape, input::RawInput::ScrollUp);
    input_map.add_action_event(input::Action::ShrinkToolShape, input::RawInput::ScrollDown);

    let mut input_state = InputState::default();

    while window.is_open() && !window.is_key_pressed(Key::Escape, minifb::KeyRepeat::No) {
        input_state.update(&window, &input_map);

        game.tick(&input_state);
        game.render(&mut screen_buffer, &input_state);

        window
            .update_with_buffer(&screen_buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
