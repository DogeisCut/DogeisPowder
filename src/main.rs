use minifb::{Key, Window, WindowOptions};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Color {
    value: u32
} impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: Option<u8>) -> Self {

        let alpha = a.unwrap_or(255) as u32;

        Self {
            value: (alpha << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
        }
    }

    pub fn a(&self) -> u8 {
        ((self.value >> 24) & 0xFF) as u8
    }

    pub fn r(&self) -> u8 {
        ((self.value >> 16) & 0xFF) as u8
    }

    pub fn g(&self) -> u8 {
        ((self.value >> 8) & 0xFF) as u8
    }

    pub fn b(&self) -> u8 {
        (self.value & 0xFF) as u8
    }

    pub fn set_a(&mut self, a: u8) {
        self.value = (self.value & 0x00FFFFFF) | ((a as u32) << 24);
    }

    pub fn set_r(&mut self, r: u8) {
        self.value = (self.value & 0xFF00FFFF) | ((r as u32) << 16);
    }

    pub fn set_g(&mut self, g: u8) {
        self.value = (self.value & 0xFFFF00FF) | ((g as u32) << 8);
    }

    pub fn set_b(&mut self, b: u8) {
        self.value = (self.value & 0xFFFFFF00) | (b as u32);
    }

    pub fn mix(&self, other: Color, factor: f32) -> Self {
        let f = factor.clamp(0.0, 1.0);
        let inv_f = 1.0 - f;

        // Mix each channel linearly (LERP)
        let r = ((self.r() as f32 * inv_f) + (other.r() as f32 * f)) as u8;
        let g = ((self.g() as f32 * inv_f) + (other.g() as f32 * f)) as u8;
        let b = ((self.b() as f32 * inv_f) + (other.b() as f32 * f)) as u8;
        let a = ((self.a() as f32 * inv_f) + (other.a() as f32 * f)) as u8;

        Color::new(r, g, b, Some(a))
    }

    pub fn overlay(&self, other: Color) -> Self {
        let r_bg = self.r() as f32 / 255.0;
        let g_bg = self.g() as f32 / 255.0;
        let b_bg = self.b() as f32 / 255.0;
        let a_bg = self.a() as f32 / 255.0;

        let r_fg = other.r() as f32 / 255.0;
        let g_fg = other.g() as f32 / 255.0;
        let b_fg = other.b() as f32 / 255.0;
        let a_fg = other.a() as f32 / 255.0;

        let a_out = a_fg + a_bg * (1.0 - a_fg);

        if a_out == 0.0 {
            return Color::new(0, 0, 0, Some(0));
        }

        let r_out = (r_fg * a_fg + r_bg * a_bg * (1.0 - a_fg)) / a_out;
        let g_out = (g_fg * a_fg + g_bg * a_bg * (1.0 - a_fg)) / a_out;
        let b_out = (b_fg * a_fg + b_bg * a_bg * (1.0 - a_fg)) / a_out;

        Color::new(
            (r_out * 255.0) as u8,
            (g_out * 255.0) as u8,
            (b_out * 255.0) as u8,
            Some((a_out * 255.0) as u8),
        )
    }
}

enum ElementKind {
    Powder,
    Liquid,
    Gas,
    Solid,
    Energy
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Element {
    Sand,
    Water,
    Wood,
} impl Element {
    pub fn density(&self) -> f32 {
        match self {
            // These numbers are based on real life g/cm3
            Element::Sand => 1.5,
            Element::Water => 1.0,
            Element::Wood => 0.75,
        }
    }
    pub fn kind(&self) -> ElementKind {
        match self {
            Element::Sand => ElementKind::Powder,
            Element::Water => ElementKind::Liquid,
            Element::Wood => ElementKind::Solid,
        }
    }
    pub fn color(&self) -> Color {
        match self {
            Element::Sand => Color::new(255, 229, 125, None),
            Element::Water => Color::new(36, 116, 255, None),
            Element::Wood => Color::new(89, 75, 51, None),
        }
    }
    pub fn brightness_varry(&self) -> u8 {
        match self {
            Element::Sand => 10,
            Element::Water => 0,
            Element::Wood => 5,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    element: Element,
    decoration: Color
} impl Particle {
    pub fn new(x: f32, y: f32, vx: f32, vy: f32, element: Element) -> Self {
        let decoration = {
            let mut new_color = element.color();
            let br = element.brightness_varry() as i16;

            if br > 0 {
                let offset_r = rand::random_range(-br..br);
                let new_r = ((new_color.r() as i16) + offset_r).clamp(0, 255) as u8;
                new_color.set_r(new_r);

                let offset_g = rand::random_range(-br..br);
                let new_g = ((new_color.g() as i16) + offset_g).clamp(0, 255) as u8;
                new_color.set_g(new_g);

                let offset_b = rand::random_range(-br..br);
                let new_b = ((new_color.b() as i16) + offset_b).clamp(0, 255) as u8;
                new_color.set_b(new_b);
            }
            
            new_color
        };

        Self {
            x,
            y,
            vx,
            vy,
            element,
            decoration,
        }
    }
    pub fn get_grid_x(&self) -> usize {
        (self.x + 0.5) as usize
    }
    pub fn get_grid_y(&self) -> usize {
        (self.y + 0.5) as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum CellState {
    Empty,
    Occupied(usize),
    OutOfBounds
}

struct IndexGrid {
    width: usize,
    height: usize,
    cells: Vec<Option<usize>>,
} impl IndexGrid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![None; width * height],
        }
    }

    pub fn get(&self, x: isize, y: isize) -> CellState {
        if x < 0 || y < 0 || x >= self.width as isize || y >= self.height as isize {
            return CellState::OutOfBounds;
        }

        match self.cells[x as usize + (y as usize * self.width)] {
            Some(index) => CellState::Occupied(index),
            None => CellState::Empty,
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: Option<usize>) {
        self.cells[x + (y * self.width)] = value;
    }

    pub fn empty(&mut self) {
        self.cells = vec![None; self.width * self.height];
    }
}

struct World {
    particles_flat: Vec<Particle>,
    particles_grid: IndexGrid,
} impl World {
    pub fn new() -> Self {
        Self {
            particles_flat: Vec::new(),
            particles_grid: IndexGrid::new(WIDTH, HEIGHT),
        }
    }

    pub fn populate_grid(&mut self) {
        self.particles_grid.empty();
        
        for index in 0..self.particles_flat.len() {
            let particle = self.particles_flat[index];
            
            let grid_x = particle.get_grid_x();
            let grid_y = particle.get_grid_y();
            
            self.particles_grid.set(grid_x, grid_y, Some(index));
        }
    }

    pub fn try_move_particle(&mut self, index: usize, target_x: f32, target_y: f32) -> bool {
        let target_grid_x = (target_x + 0.5) as isize;
        let target_grid_y = (target_y + 0.5) as isize;

        match self.particles_grid.get(target_grid_x, target_grid_y) {
            CellState::Empty => {
                let mut particle = self.particles_flat[index];

                let old_grid_x = particle.get_grid_x();
                let old_grid_y = particle.get_grid_y();

                particle.x = target_x;
                particle.y = target_y;

                self.particles_flat[index] = particle;
                
                self.particles_grid.set(old_grid_x, old_grid_y, None);
                self.particles_grid.set(target_grid_x as usize, target_grid_y as usize, Some(index));

                true
            },
            CellState::Occupied(other_index) => {
                let mut particle = self.particles_flat[index];
                let mut other_particle = self.particles_flat[other_index];

                if particle.element.density() <= other_particle.element.density() {
                    return false;
                }

                let old_grid_x = particle.get_grid_x();
                let old_grid_y = particle.get_grid_y();

                let old_x = particle.x;
                let old_y = particle.y;

                particle.x = target_x;
                particle.y = target_y;

                other_particle.x = old_x;
                other_particle.y = old_y;

                self.particles_flat[index] = particle;
                self.particles_flat[other_index] = other_particle;
                
                self.particles_grid.set(target_grid_x as usize, target_grid_y as usize, Some(index));
                self.particles_grid.set(old_grid_x, old_grid_y, Some(other_index));
                
                true
            },
            CellState::OutOfBounds => false,
        }
    }

    pub fn update_physics(&mut self) {
        for index in 0..self.particles_flat.len() {
            let mut particle = self.particles_flat[index];


            match particle.element.kind() {
                ElementKind::Powder => {
                    let bias: f32 = if rand::random::<bool>() { 1.0 } else { -1.0 };

                    if !self.try_move_particle(index, particle.x, particle.y + 1.0) {
                        if !self.try_move_particle(index, particle.x + bias, particle.y + 1.0) {
                            self.try_move_particle(index, particle.x - bias, particle.y + 1.0);
                        };
                    };
                },
                ElementKind::Liquid => {
                    let bias: f32 = if rand::random::<bool>() { 1.0 } else { -1.0 };

                    if !self.try_move_particle(index, particle.x, particle.y + 1.0) {
                        if !self.try_move_particle(index, particle.x + bias, particle.y + 1.0) {
                            if !self.try_move_particle(index, particle.x - bias, particle.y + 1.0) {
                                if !self.try_move_particle(index, particle.x + bias, particle.y) {
                                    self.try_move_particle(index, particle.x - bias, particle.y);
                                };
                            };
                        };
                    };
                },
                ElementKind::Gas => {
                    let bias: f32 = if rand::random::<bool>() { 1.0 } else { -1.0 };

                    if !self.try_move_particle(index, particle.x + bias, particle.y - 1.0) {
                        if !self.try_move_particle(index, particle.x, particle.y - 1.0) {
                            if !self.try_move_particle(index, particle.x + bias, particle.y - 1.0) {
                                if !self.try_move_particle(index, particle.x - bias, particle.y - 1.0) {
                                    if !self.try_move_particle(index, particle.x + bias, particle.y) {
                                        self.try_move_particle(index, particle.x - bias, particle.y);
                                    };
                                };
                            };
                        };
                    };
                },
                ElementKind::Solid => {
                    // Solids sit still, they don't even move with velocity.
                },
                ElementKind::Energy => {
                    // Normally energy particles would move with velocity but we don't have that implemented.
                },
            }
        } 
    }
}

const WIDTH: usize = 320;
const HEIGHT: usize = 180;
const BG_COLOR: Color = Color { value: 0xFF1A1A1A };

fn main() {
    let mut world: World = World::new();

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
                if let CellState::Occupied(index) = world.particles_grid.get(x as isize, y as isize) {
                    let particle = world.particles_flat[index];
                    let color = particle.element.color().overlay(particle.decoration);

                    screen_buffer[x + (y * WIDTH)] = BG_COLOR.overlay(color).value;
                }
            }
        }

        window.update_with_buffer(&screen_buffer, WIDTH, HEIGHT).unwrap();
    }
}
