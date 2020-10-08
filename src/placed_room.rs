// External includes.

// Standard includes.
use std::ops::{Index, IndexMut};

// Internal includes.
use super::{Room, TileType};
use crate::geometry::{Length, LocalPosition, PlacedShape, Position};

pub trait PlacedRoom<'a>: PlacedShape + Room<'a> {
    fn tile_type_at(&self, pos: Position) -> Option<&TileType> {
        let pos = pos - *self.pos();
        if pos.x() < 0 || pos.y() < 0 {
            None
        } else {
            let local_pos = LocalPosition::new(pos.x() as Length, pos.y() as Length);
            self.tile_type_at_local(local_pos)
        }
    }

    fn tile_type_at_mut(&mut self, pos: Position) -> Option<&mut TileType> {
        let pos = pos - *self.pos();
        if pos.x() < 0 || pos.y() < 0 {
            None
        } else {
            let local_pos = LocalPosition::new(pos.x() as Length, pos.y() as Length);
            self.tile_type_at_local_mut(local_pos)
        }
    }
}

impl<'a> Index<Position> for dyn PlacedRoom<'a> {
    type Output = TileType;

    fn index(&self, pos: Position) -> &Self::Output {
        self.tile_type_at(pos).unwrap()
    }
}

impl<'a> IndexMut<Position> for dyn PlacedRoom<'a> {
    fn index_mut(&mut self, pos: Position) -> &mut Self::Output {
        self.tile_type_at_mut(pos).unwrap()
    }
}
