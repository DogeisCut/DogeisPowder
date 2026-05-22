use crate::{color::Color, element::Element};

pub enum Shape {
    Oval(usize, usize, f32),
    Rect(usize, usize, f32),
    Tri(usize, usize, f32),
}

pub enum Kind {
    Smear(Option<Element>),
    Replace(Option<Element>, Option<Element>),
    Grab(),
    Temperature(f32),
    Color(Option<Color>),
}

pub struct Tool {
    pub shape: Shape,
    pub kind: Kind,
}
impl Tool {
    pub fn new(shape: Shape, kind: Kind) -> Self {
        Self { shape, kind }
    }
}
