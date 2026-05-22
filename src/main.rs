use minifb::{Key, Window, WindowOptions};
use crate::element::Element;
use crate::particle::Particle;
use crate::world::World;
use crate::color::Color;

mod color;
mod element;
mod particle;
mod world;
mod tool;

const WIDTH: usize = 320;
const HEIGHT: usize = 180;
const BG_COLOR: Color = Color { value: 0xFF1A1A1A };

fn main() {
    let mut world: World = World::new(WIDTH, HEIGHT);

    let mut window = Window::new(
        "DogeisPowder",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: minifb::Scale::X4,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut screen_buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    while window.is_open() && !window.is_key_pressed(Key::Escape, minifb::KeyRepeat::No) {
        world.particles_flat.push(
            Particle::new(100.0, 100.0, 0.0, 0.0, Element::Sand)
        );

        world.particles_flat.push(
            Particle::new(200.0, 100.0, 0.0, 0.0, Element::Water)
        );

        world.populate_grid();
        world.update_physics();

        for pixel in screen_buffer.iter_mut() {
            *pixel = BG_COLOR.value
        }

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if let world::CellState::Occupied(index) = world.particles_grid.get(x as isize, y as isize) {
                    let particle = world.particles_flat[index];
                    let color = particle.element.color().overlay(particle.decoration);

                    screen_buffer[x + (y * WIDTH)] = BG_COLOR.overlay(color).value;
                }
            }
        }

        window.update_with_buffer(&screen_buffer, WIDTH, HEIGHT).unwrap();
    }
}
