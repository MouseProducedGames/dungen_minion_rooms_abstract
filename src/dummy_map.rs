// External includes.

// Standard includes.

// Internal includes.
use super::*;
use crate::geometry::*;

#[derive(Clone)]
pub(crate) struct DummyMap {
    area: Area,
}

impl DummyMap {
    pub(crate) fn new() -> Self {
        Self {
            area: Area::new(Position::zero(), Size::zero()),
        }
    }
}

impl HasArea for DummyMap {
    fn area(&self) -> &Area {
        &self.area
    }

    fn area_mut(&mut self) -> &mut Area {
        &mut self.area
    }
}

impl HasPosition for DummyMap {
    fn position(&self) -> &Position {
        self.area.position()
    }

    fn position_mut(&mut self) -> &mut Position {
        self.area.position_mut()
    }
}

impl HasSize for DummyMap {
    fn size(&self) -> &Size {
        self.area.size()
    }

    fn size_mut(&mut self) -> &mut Size {
        self.area.size_mut()
    }
}

impl IntersectsPos for DummyMap {
    fn intersects_pos(&self, position: Position) -> bool {
        false
    }
}

impl Map for DummyMap {
    fn box_clone(&self) -> Box<dyn Map> {
        Box::new((*self).clone())
    }

    fn map_id(&self) -> MapId {
        0
    }

    fn tile_type_at_local(&self, pos: Position) -> Option<&TileType> {
        None
    }

    fn tile_type_at_local_mut(&mut self, pos: Position) -> Option<&mut TileType> {
        None
    }

    fn tile_type_at_local_set(&mut self, pos: Position, tile_type: TileType) -> Option<TileType> {
        None
    }
}

impl Placed for DummyMap {}

impl PlacedObject for DummyMap {}

impl PlacedShape for DummyMap {}

impl PortalCollection for DummyMap {
    fn add_portal(
        &mut self,
        local_position: Position,
        portal_to_room_facing: OrdinalDirection,
        portal_to_room_position: Position,
        target: MapId,
    ) {
    }

    fn get_portal_at(&self, index: usize) -> Option<&Portal> {
        None
    }

    fn get_portal_at_mut(&mut self, index: usize) -> Option<&mut Portal> {
        None
    }

    fn portal_count(&self) -> usize {
        0
    }

    fn portals(&self) -> Portals {
        Portals::new(&[])
    }

    fn portals_mut(&mut self) -> PortalsMut {
        PortalsMut::new(&mut [])
    }
}

impl Shape for DummyMap {}

impl SubRoomCollection for DummyMap {
    fn add_sub_room(&mut self, local_position: Position, target: MapId) {}

    fn get_sub_room_at(&self, index: usize) -> Option<&SubRoom> {
        None
    }

    fn get_sub_room_at_mut(&mut self, index: usize) -> Option<&mut SubRoom> {
        None
    }

    fn sub_room_count(&self) -> usize {
        0
    }

    fn sub_rooms(&self) -> SubRooms {
        SubRooms::new(&[])
    }

    fn sub_rooms_mut(&mut self) -> SubRoomsMut {
        SubRoomsMut::new(&mut [])
    }
}
