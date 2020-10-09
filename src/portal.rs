// External includes.

// Standard includes.

// Internal includes.
use super::PlacedRoom;
use crate::geometry::*;

#[derive(Clone)]
pub struct Portal {
    local: LocalPosition,
    portal_to_room_facing: OrdinalDirection,
    target: Box<dyn PlacedRoom>,
}

impl Portal {
    pub fn new(
        local: LocalPosition,
        portal_to_room_facing: OrdinalDirection,
        target: Box<dyn PlacedRoom>,
    ) -> Self {
        Self {
            local,
            portal_to_room_facing,
            target,
        }
    }

    #[allow(clippy::borrowed_box)]
    pub fn target(&self) -> &Box<dyn PlacedRoom> {
        &self.target
    }

    pub fn portal_to_room_facing(&self) -> &OrdinalDirection {
        &self.portal_to_room_facing
    }

    pub fn portal_to_room_facing_mut(&mut self) -> &mut OrdinalDirection {
        &mut self.portal_to_room_facing
    }

    pub fn target_mut(&mut self) -> &mut Box<dyn PlacedRoom> {
        &mut self.target
    }
}

impl HasLocalPosition for Portal {
    fn local(&self) -> &LocalPosition {
        &self.local
    }

    fn local_mut(&mut self) -> &mut LocalPosition {
        &mut self.local
    }
}
