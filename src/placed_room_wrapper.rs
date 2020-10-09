// External includes.

// Standard includes.

// Internal includes.
use super::*;
use crate::geometry::*;

#[derive(Clone)]
pub struct PlacedRoomWrapper {
    area: Area,
    room: Box<dyn Room>,
}

impl PlacedRoomWrapper {
    pub fn new<TRoom: 'static>(pos: Position, room: TRoom) -> Self
    where
        TRoom: Room,
    {
        Self {
            area: Area::new(pos, *room.size()),
            room: Box::new(room),
        }
    }
}

impl HasLocalPosition for PlacedRoomWrapper {
    fn local(&self) -> &LocalPosition {
        self.room.local()
    }

    fn local_mut(&mut self) -> &mut LocalPosition {
        self.room.local_mut()
    }
}

impl HasArea for PlacedRoomWrapper {
    fn area(&self) -> &Area {
        &self.area
    }

    fn area_mut(&mut self) -> &mut Area {
        &mut self.area
    }
}

impl HasPosition for PlacedRoomWrapper {
    fn pos(&self) -> &Position {
        self.area.pos()
    }

    fn pos_mut(&mut self) -> &mut Position {
        self.area.pos_mut()
    }
}

impl IntersectsLocalPos for PlacedRoomWrapper {
    fn intersects_local_pos(&self, pos: LocalPosition) -> bool {
        self.room.intersects_local_pos(pos)
    }
}

impl IntersectsPos for PlacedRoomWrapper {
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

impl Placed for PlacedRoomWrapper {}

impl PlacedRoom for PlacedRoomWrapper {
    fn box_placed_clone(&self) -> Box<dyn PlacedRoom> {
        Box::new((*self).clone())
    }
}

impl PlacedShape for PlacedRoomWrapper {}

impl PlacedObject for PlacedRoomWrapper {}

impl PortalCollection for PlacedRoomWrapper {
    fn add_portal(
        &mut self,
        local: LocalPosition,
        portal_to_room_facing: OrdinalDirection,
        target: Box<dyn PlacedRoom>,
    ) {
        Room::add_portal(self, local, portal_to_room_facing, target)
    }

    fn get_portal_at(&self, index: usize) -> Option<&Portal> {
        Room::get_portal_at(self, index)
    }

    fn get_portal_at_mut(&mut self, index: usize) -> Option<&mut Portal> {
        Room::get_portal_at_mut(self, index)
    }

    fn portal_count(&self) -> usize {
        Room::portal_count(self)
    }
}

impl Room for PlacedRoomWrapper {
    fn box_clone(&self) -> Box<dyn Room> {
        Box::new((*self).clone())
    }

    fn portals(&self) -> Portals {
        self.room.portals()
    }
    fn portals_mut(&mut self) -> PortalsMut {
        self.room.portals_mut()
    }

    fn sub_rooms(&self) -> SubRooms {
        self.room.sub_rooms()
    }

    fn sub_rooms_mut(&mut self) -> SubRoomsMut {
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

impl Shape for PlacedRoomWrapper {}

impl SubRoomCollection for PlacedRoomWrapper {
    fn add_sub_room(&mut self, local: LocalPosition, target: Box<dyn Room>) {
        Room::add_sub_room(self, local, target)
    }

    fn get_sub_room_at(&self, index: usize) -> Option<&SubRoom> {
        Room::get_sub_room_at(self, index)
    }

    fn get_sub_room_at_mut(&mut self, index: usize) -> Option<&mut SubRoom> {
        Room::get_sub_room_at_mut(self, index)
    }

    fn sub_room_count(&self) -> usize {
        Room::sub_room_count(self)
    }
}

impl HasSize for PlacedRoomWrapper {
    fn size(&self) -> &Size {
        self.area.size()
    }

    fn size_mut(&mut self) -> &mut Size {
        self.area.size_mut()
    }
}
