// External includes.

// Standard includes.
use std::ops::{Index, IndexMut};

// Internal includes.
use super::*;
use crate::geometry::*;

pub struct PlacedRoomWrapper<'a> {
    area: Area,
    room: Box<dyn Room<'a>>,
}

impl<'a> PlacedRoomWrapper<'a> {
    pub fn new<TRoom: 'static>(pos: Position, room: TRoom) -> Self
    where
        TRoom: Room<'a>,
    {
        Self {
            area: Area::new(pos, *room.size()),
            room: Box::new(room),
        }
    }
}

impl<'a> HasLocalPosition for PlacedRoomWrapper<'a> {
    fn local(&self) -> &LocalPosition {
        self.room.local()
    }

    fn local_mut(&mut self) -> &mut LocalPosition {
        self.room.local_mut()
    }
}

impl<'a> HasArea for PlacedRoomWrapper<'a> {
    fn area(&self) -> &Area {
        &self.area
    }

    fn area_mut(&mut self) -> &mut Area {
        &mut self.area
    }
}

impl<'a> HasPosition for PlacedRoomWrapper<'a> {
    fn pos(&self) -> &Position {
        self.area.pos()
    }

    fn pos_mut(&mut self) -> &mut Position {
        self.area.pos_mut()
    }
}

impl<'a> IntersectsLocalPos for PlacedRoomWrapper<'a> {
    fn intersects_local_pos(&self, pos: LocalPosition) -> bool {
        self.room.intersects_local_pos(pos)
    }
}

impl<'a> IntersectsPos for PlacedRoomWrapper<'a> {
    fn intersects_pos(&self, pos: Position) -> bool {
        let rel_pos = pos - *self.pos();
        if rel_pos.x() < 0 || rel_pos.y() < 0 {
            false
        } else {
            let local = LocalPosition::new(rel_pos.x() as Length, rel_pos.y() as Length);
            self.intersects_local_pos(local)
        }
    }
}

impl<'a> Placed for PlacedRoomWrapper<'a> {}

impl<'a> PlacedRoom<'a> for PlacedRoomWrapper<'a> {}

impl<'a> PlacedShape for PlacedRoomWrapper<'a> {}

impl<'a> PlacedObject for PlacedRoomWrapper<'a> {}

impl<'a> Room<'a> for PlacedRoomWrapper<'a> {
    fn portals(&'a self) -> Portals<'a> {
        self.room.portals()
    }
    fn portals_mut(&'a mut self) -> PortalsMut<'a> {
        self.room.portals_mut()
    }

    fn sub_rooms(&'a self) -> SubRooms<'a> {
        self.room.sub_rooms()
    }

    fn sub_rooms_mut(&'a mut self) -> SubRoomsMut<'a> {
        self.room.sub_rooms_mut()
    }

    fn tile_type_at_local(&self, pos: LocalPosition) -> Option<&TileType> {
        self.room.tile_type_at_local(pos)
    }

    fn tile_type_at_local_mut(&mut self, pos: LocalPosition) -> Option<&mut TileType> {
        *self.room.size_mut() = *self.size();
        self.room.tile_type_at_local_mut(pos)
    }

    fn tile_type_at_local_set(
        &mut self,
        pos: LocalPosition,
        tile_type: TileType,
    ) -> Option<TileType> {
        *self.room.size_mut() = *self.size();
        self.room.tile_type_at_local_set(pos, tile_type)
    }
}

impl<'a> Shape for PlacedRoomWrapper<'a> {}

impl<'a> HasSize for PlacedRoomWrapper<'a> {
    fn size(&self) -> &Size {
        self.area.size()
    }

    fn size_mut(&mut self) -> &mut Size {
        self.area.size_mut()
    }
}
