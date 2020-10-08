// External includes.

// Standard includes.

// Internal includes.
use super::{PlacedShape, Position, Room, TileType};

pub trait PlacedRoom: PlacedShape + Room {
    fn tile_type_at(&self, pos: Position) -> &TileType;
}
