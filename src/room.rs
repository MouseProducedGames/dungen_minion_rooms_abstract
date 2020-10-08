// External includes.

// Standard includes.
use std::ops::{Index, IndexMut};

// Internal includes.
use super::{
    HasLocalPosition, LocalPosition, Portals, PortalsMut, Shape, SubRooms, SubRoomsMut, TileType,
};

pub trait Room<'a>: HasLocalPosition + Shape {
    fn portals(&'a self) -> Portals<'a>;

    fn portals_mut(&'a mut self) -> PortalsMut<'a>;

    fn sub_rooms(&'a self) -> SubRooms<'a>;

    fn sub_rooms_mut(&'a mut self) -> SubRoomsMut<'a>;

    fn tile_type_at_local(&self, pos: LocalPosition) -> Option<&TileType>;

    fn tile_type_at_local_mut(&mut self, pos: LocalPosition) -> Option<&mut TileType>;

    fn tile_type_at_local_set(
        &mut self,
        pos: LocalPosition,
        tile_type: TileType,
    ) -> Option<TileType>;
}

impl<'a> Index<LocalPosition> for dyn Room<'a> {
    type Output = TileType;

    fn index(&self, pos: LocalPosition) -> &Self::Output {
        self.tile_type_at_local(pos).unwrap()
    }
}

impl<'a> IndexMut<LocalPosition> for dyn Room<'a> {
    fn index_mut(&mut self, pos: LocalPosition) -> &mut Self::Output {
        self.tile_type_at_local_mut(pos).unwrap()
    }
}
