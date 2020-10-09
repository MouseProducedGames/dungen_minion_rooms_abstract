// External includes.

// Standard includes.
use std::ops::{Index, IndexMut};

// Internal includes.
use super::{Room, TileType};
use crate::geometry::*;

/// The defining trait for a map that has a world position.
///
/// `PlacedRoom` derives from [`PlacedShape`](geometry/struct.PlacedShape.html) to define its world position and shape, and derives from [`Room`](trait.Room.html) to define its function as a map.
pub trait PlacedRoom: PlacedShape + Room {
    /// A helper method to allow structs implementing `PlacedRoom` to be `Clone`'ed.
    ///
    /// [https://users.rust-lang.org/t/solved-is-it-possible-to-clone-a-boxed-trait-object/1714/5](https://users.rust-lang.org/t/solved-is-it-possible-to-clone-a-boxed-trait-object/1714/5)
    fn box_placed_clone(&self) -> Box<dyn PlacedRoom>;

    /// Gets an option for an immutable reference to the `TileType` at the given `Position`. Returns None if the `Position` is out of bounds, or there is no tile at that location.
    ///
    /// This method has a default implementation.
    fn tile_type_at(&self, pos: Position) -> Option<&TileType> {
        let pos = pos - *self.pos();
        if pos.x() < 0 || pos.y() < 0 {
            None
        } else {
            let local_pos = LocalPosition::new(pos.x() as Length, pos.y() as Length);
            self.tile_type_at_local(local_pos)
        }
    }

    /// Gets an option for a mutable reference to the `TileType` at the given `Position`. Returns None if the `Position` is out of bounds, or there is no tile at that location.
    /// 
    /// This method has a default implementation.
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

impl Clone for Box<dyn PlacedRoom> {
    fn clone(&self) -> Box<dyn PlacedRoom> {
        self.box_placed_clone()
    }
}

impl Index<Position> for dyn PlacedRoom {
    type Output = TileType;

    fn index(&self, pos: Position) -> &Self::Output {
        self.tile_type_at(pos).unwrap()
    }
}

impl IndexMut<Position> for dyn PlacedRoom {
    fn index_mut(&mut self, pos: Position) -> &mut Self::Output {
        self.tile_type_at_mut(pos).unwrap()
    }
}
