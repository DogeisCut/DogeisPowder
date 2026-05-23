use rand::seq::index;

use crate::{
    color::Color,
    particle::Particle,
    world::{self, World},
};

pub enum StateOfMatter {
    Powder,
    Liquid,
    Gas,
    Solid,
    Energy,
}

pub enum ValueReaction<T> {
    None,
    Transition(T, Option<Element>, Option<Element>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Element {
    Sand,
    Water,
    Wood,
    Ice,
}

impl Element {
    pub fn name(&self) -> &'static str {
        match self {
            Element::Sand => "Sand",
            Element::Water => "Water",
            Element::Wood => "Wood",
            Element::Ice => "Ice",
        }
    }
    pub fn density(&self) -> f32 {
        match self {
            // These numbers are based on real life g/cm3
            Element::Sand => 1.5,
            Element::Water => 1.0,
            Element::Wood => 0.75,
            Element::Ice => 0.934,
        }
    }
    pub fn kind(&self) -> StateOfMatter {
        match self {
            Element::Sand => StateOfMatter::Powder,
            Element::Water => StateOfMatter::Liquid,
            Element::Wood => StateOfMatter::Solid,
            Element::Ice => StateOfMatter::Solid,
        }
    }
    pub fn color(&self) -> Color {
        match self {
            Element::Sand => Color::new(255, 229, 125, None),
            Element::Water => Color::new(36, 116, 255, None),
            Element::Wood => Color::new(89, 75, 51, None),
            Element::Ice => Color::new(128, 185, 255, None),
        }
    }
    pub fn brightness_vary(&self) -> u8 {
        match self {
            Element::Sand => 10,
            Element::Wood => 5,
            _ => 0,
        }
    }
    pub fn temperature_kelvin(&self) -> f32 {
        match self {
            Element::Ice => 273.15,
            _ => 298.15,
        }
    }
    pub fn low_temperature_reaction(&self) -> ValueReaction<f32> {
        match self {
            Element::Water => {
                ValueReaction::Transition(273.15, Some(Element::Ice), Some(Element::Water))
            }
            _ => ValueReaction::None,
        }
    }
    pub fn high_temperature_reaction(&self) -> ValueReaction<f32> {
        match self {
            Element::Sand => ValueReaction::None, //TODO: ValueReaction::Transition(1_986.0, Some(Element::Lava), Some(Element::Glass))
            Element::Water => ValueReaction::None, //TODO: ValueReaction::Transition(373.15, Some(Element::Steam), Some(Element::Water))
            Element::Wood => ValueReaction::None, //TODO: ValueReaction::Transition(612.0, Some(Element::Charcoal), None) // Further heated charcoal turns to ash at 723.0
            Element::Ice => {
                ValueReaction::Transition(273.15, Some(Element::Water), Some(Element::Ice))
            }
            _ => ValueReaction::None,
        }
    }
    pub fn ionization_temperature(&self) -> Option<f32> {
        // this is only an option because plasma itself wont be ionizing
        match self {
            Element::Sand => Some(22_500.0),
            Element::Water => Some(12_000.0),
            Element::Wood => Some(10_000.0),
            Element::Ice => Some(12_000.0),
            _ => Some(15000.0),
        }
    }
    pub fn life(&self) -> Option<f32> {
        match self {
            _ => None,
        }
    }
    pub fn flamability(&self) -> usize {
        match self {
            Element::Wood => 2,
            _ => 0,
        }
    }

    pub fn pre_tick(&self, index: usize, particle: Particle, world: &mut World) {
        match self {
            _ => {}
        }
    }
    pub fn physics_tick(&self, index: usize, particle: Particle, world: &mut World) {
        match self {
            _ => {}
        }
    }
    pub fn post_tick(&self, index: usize, particle: Particle, world: &mut World) {
        match self {
            _ => {}
        }
    }
    pub fn on_physics_move_attempt(
        &self,
        index: usize,
        particle: Particle,
        world: &mut World,
        new_x: f32,
        new_y: f32,
    ) {
        match self {
            _ => {}
        }
    }
}
