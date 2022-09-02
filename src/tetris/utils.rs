use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn add(&self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn sub(&self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn mul(&self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

pub fn pos(x: i32, y: i32) -> Pos {
    Pos { x, y }
}

pub fn rotate_around(point: Pos, anchor: Pos) -> Pos {
    let mut rotated = point;
    rotated = rotated.sub(anchor);
    return pos(-rotated.y, rotated.x).add(anchor);
}

pub fn darken_colour(colour: Color) -> Color {
    let mut colour = colour;
    colour.r *= 0.75;
    colour.g *= 0.75;
    colour.b *= 0.75;
    colour.a = 100.0;
    colour
}