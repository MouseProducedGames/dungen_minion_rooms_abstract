// External includes.

// Standard includes.
use std::ops::{Add, Sub};

// Internal includes.
use super::{Coord, HasPosition};

#[derive(Copy, Clone, Debug)]
pub struct Position {
    x: Coord,
    y: Coord,
}

impl Position {
    pub fn new(x: Coord, y: Coord) -> Self {
        Self { x, y }
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl HasPosition for Position {
    fn pos(&self) -> &Self {
        self
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
