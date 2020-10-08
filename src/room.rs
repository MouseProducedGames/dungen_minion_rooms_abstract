// External includes.

// Standard includes.

// Internal includes.
use super::{HasLocalPosition, LocalPosition, Portals, Shape, SubRooms, TileType};

pub trait Room: HasLocalPosition + Shape {
    fn local(&self) -> &LocalPosition;

    fn portals(&self) -> Portals;

    fn sub_rooms(&self) -> SubRooms;

    fn tile_type_at_local_pos(&self, pos: LocalPosition) -> &TileType;
}
