use crate::color::Color;

pub enum StateOfMatter {
    Powder,
    Liquid,
    Gas,
    Solid,
    Energy
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Element {
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
    pub fn kind(&self) -> StateOfMatter {
        match self {
            Element::Sand => StateOfMatter::Powder,
            Element::Water => StateOfMatter::Liquid,
            Element::Wood => StateOfMatter::Solid,
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
    pub fn default_temperature_kelvin(&self) -> f32 {
        match self {
            Element::Sand => 298.15,
            Element::Water => 298.15,
            Element::Wood => 298.15,
        }
    }
}