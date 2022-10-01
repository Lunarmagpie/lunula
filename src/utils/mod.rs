use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Vec2D {
    pub x: i16,
    pub y: i16,
}

impl Vec2D {
    pub fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }
}

impl Vec2D {
    pub fn max(&self, other: Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }
}

impl ops::Add for Vec2D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl ops::Sub for Vec2D {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
