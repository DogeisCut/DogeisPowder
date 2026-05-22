use crate::{color::Color, element::Element};

pub enum Shape {
    Oval(u32, u32, f32),
    Rect(u32, u32, f32),
    Tri(u32, u32, f32),
}

pub enum Tool {
    Smear(Shape, Option<Element>),
    Replace(Shape, Option<Element>, Option<Element>),
    Grab(Shape),
    Temperature(Shape, f32),
    Color(Shape, Option<Color>),
}
