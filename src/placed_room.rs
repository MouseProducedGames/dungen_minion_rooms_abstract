// External includes.

// Standard includes.
use std::ops::{Index, IndexMut};

// Internal includes.
use super::{Portal, PortalCollection, Room, SubRoom, SubRoomCollection, TileType};
use crate::geometry::*;

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

impl<'a, TPlacedRoom> SubRoomCollection<'a> for TPlacedRoom
where
    TPlacedRoom: PlacedRoom<'a>,
{
    fn add_sub_room(&mut self, local: LocalPosition, target: &dyn Room<'a>) {
        Room::add_sub_room(self, local, target)
    }

    fn get_sub_room_at(&self, index: usize) -> Option<&SubRoom<'a>> {
        Room::get_sub_room_at(self, index)
    }

    fn get_sub_room_at_mut(&mut self, index: usize) -> Option<&mut SubRoom<'a>> {
        Room::get_sub_room_at_mut(self, index)
    }

    fn sub_room_count(&self) -> usize {
        Room::sub_room_count(self)
    }
}

impl<'a, TPlacedRoom> PortalCollection<'a> for TPlacedRoom
where
    TPlacedRoom: PlacedRoom<'a>,
{
    fn add_portal(&mut self, local: LocalPosition, target: &dyn PlacedRoom<'a>) {
        Room::add_portal(self, local, target)
    }

    fn get_portal_at(&self, index: usize) -> Option<&Portal<'a>> {
        Room::get_portal_at(self, index)
    }

    fn get_portal_at_mut(&mut self, index: usize) -> Option<&mut Portal<'a>> {
        Room::get_portal_at_mut(self, index)
    }

    fn portal_count(&self) -> usize {
        Room::portal_count(self)
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
