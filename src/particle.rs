use crate::{color::Color, element::Element};

#[derive(Clone, Copy, Debug)]
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub element: Element,
    pub decoration: Color,
    pub temperature_kelvin: f32,
    pub sub_type: Option<Element>,
    pub life: Option<f32>,
}
impl Particle {
    pub fn new(element: Element, x: f32, y: f32, vx: f32, vy: f32) -> Self {
        let decoration = {
            let mut new_color = element.color();
            let br = element.brightness_vary() as i16;

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
            temperature_kelvin: element.default_temperature_kelvin(),
            sub_type: None,
            life: element.default_life(),
        }
    }
    pub fn get_grid_x(&self) -> usize {
        (self.x + 0.5) as usize
    }
    pub fn get_grid_y(&self) -> usize {
        (self.y + 0.5) as usize
    }
}
