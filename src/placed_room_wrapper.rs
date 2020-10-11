// External includes.

// Standard includes.

// Internal includes.
use super::*;
use crate::geometry::*;

/// A wrapper which turns a [`Room`](trait.Room.html) into a [`PlacedRoom`](trait.PlacedRoom.html).
#[derive(Clone)]
pub struct PlacedRoomWrapper {
    area: Area,
    room: Box<dyn Room>,
}

impl PlacedRoomWrapper {
    /// Creates a new `PlacedRoomWrapper` from a given `Position` and `Room`.
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

impl HasArea for PlacedRoomWrapper {
    fn area(&self) -> &Area {
        &self.area
    }

    fn area_mut(&mut self) -> &mut Area {
        &mut self.area
    }
}

impl HasPosition for PlacedRoomWrapper {
    fn position(&self) -> &Position {
        self.area.position()
    }

    fn position_mut(&mut self) -> &mut Position {
        self.area.position_mut()
    }
}

impl IntersectsShapePosition for PlacedRoomWrapper {
    fn intersects_shape_position(&self, pos: ShapePosition) -> bool {
        if !pos.is_valid_shape_index() {
            false
        } else {
            self.intersects_shape_position(pos)
        }
    }
}

impl IntersectsPos for PlacedRoomWrapper {
    fn intersects_pos(&self, pos: Position) -> bool {
        self.intersects_shape_position(ShapePosition::from(pos))
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
        local_shape_position: ShapePosition,
        portal_to_room_facing: OrdinalDirection,
        target: Box<dyn PlacedRoom>,
    ) {
        self.room
            .add_portal(local_shape_position, portal_to_room_facing, target)
    }

    fn get_portal_at(&self, index: usize) -> Option<&Portal> {
        self.room.get_portal_at(index)
    }

    fn get_portal_at_mut(&mut self, index: usize) -> Option<&mut Portal> {
        self.room.get_portal_at_mut(index)
    }

    fn portal_count(&self) -> usize {
        self.room.portal_count()
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

    fn tile_type_at_local(&self, pos: ShapePosition) -> Option<&TileType> {
        self.room.tile_type_at_local(pos)
    }

    fn tile_type_at_local_mut(&mut self, pos: ShapePosition) -> Option<&mut TileType> {
        self.room.tile_type_at_local_mut(pos)
    }

    fn tile_type_at_local_set(
        &mut self,
        pos: ShapePosition,
        tile_type: TileType,
    ) -> Option<TileType> {
        if !pos.is_valid_shape_index() {
            None
        } else {
            *self.area.size_mut().height_mut() = self.area.size().height().max(pos.y() as u32 + 1);
            *self.area.size_mut().width_mut() = self.area.size().width().max(pos.x() as u32 + 1);
            self.room.tile_type_at_local_set(pos, tile_type)
        }
    }
}

impl Shape for PlacedRoomWrapper {}

impl SubRoomCollection for PlacedRoomWrapper {
    fn add_sub_room(&mut self, local_shape_position: ShapePosition, target: Box<dyn Room>) {
        self.room.add_sub_room(local_shape_position, target)
    }

    fn get_sub_room_at(&self, index: usize) -> Option<&SubRoom> {
        self.room.get_sub_room_at(index)
    }

    fn get_sub_room_at_mut(&mut self, index: usize) -> Option<&mut SubRoom> {
        self.room.get_sub_room_at_mut(index)
    }

    fn sub_room_count(&self) -> usize {
        self.room.sub_room_count()
    }
}

impl HasSize for PlacedRoomWrapper {
    fn size(&self) -> &Size {
        self.room.size()
    }

    fn size_mut(&mut self) -> &mut Size {
        self.room.size_mut()
    }
}
