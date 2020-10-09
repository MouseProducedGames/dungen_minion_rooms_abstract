// External includes.

// Standard includes.
use std::ops::{Index, IndexMut};

// Internal includes.
use super::{
    PortalCollection, Portals, PortalsMut, SubRoomCollection, SubRooms, SubRoomsMut,
    TileType,
};
use crate::geometry::{HasLocalPosition, LocalPosition, Shape};

pub trait Room: HasLocalPosition + PortalCollection + Shape + SubRoomCollection {
    fn box_clone(&self) -> Box<dyn Room>;
    
    fn portals(&self) -> Portals;

    fn portals_mut(&mut self) -> PortalsMut;

    fn sub_rooms(&self) -> SubRooms;

    fn sub_rooms_mut(&mut self) -> SubRoomsMut;

    fn tile_type_at_local(&self, pos: LocalPosition) -> Option<&TileType>;

    fn tile_type_at_local_mut(&mut self, pos: LocalPosition) -> Option<&mut TileType>;

    fn tile_type_at_local_set(
        &mut self,
        pos: LocalPosition,
        tile_type: TileType,
    ) -> Option<TileType>;
}

impl Clone for Box<dyn Room> {
    fn clone(&self) -> Box<dyn Room> {
        self.box_clone()
    }
}

impl Index<LocalPosition> for dyn Room {
    type Output = TileType;

    fn index(&self, pos: LocalPosition) -> &Self::Output {
        self.tile_type_at_local(pos).unwrap()
    }
}

impl IndexMut<LocalPosition> for dyn Room {
    fn index_mut(&mut self, pos: LocalPosition) -> &mut Self::Output {
        self.tile_type_at_local_mut(pos).unwrap()
    }
}
