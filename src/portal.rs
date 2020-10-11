// External includes.

// Standard includes.

// Internal includes.
use super::PlacedRoom;
use crate::geometry::*;

/// Contains information about a [`PlacedRoom`](trait.PlacedRoom.html) that can be reached from this `Portal`.
///
/// `Portal` contains a [`ShapePosition`](geometry/struct.ShapePosition.html), which designates where the `Portal` is on the map the `Portal` originates from, an [`OrdinalDirection`](geometry/enum.OrdinalDirection.html) which designates in which direction the target room faces from the perspective of the portal, and a `Box<dyn PlacedRoom>` target room.
#[derive(Clone)]
pub struct Portal {
    local_shape_position: ShapePosition,
    portal_to_room_facing: OrdinalDirection,
    target: Box<dyn PlacedRoom>,
}

impl Portal {
    /// Creates a new `Portal` at a given `ShapePosition`, where the `PlacedRoom` is facing a specific `OrdinalDirection` from the `Portal`'s perspective.
    pub fn new(
        local_shape_position: ShapePosition,
        portal_to_room_facing: OrdinalDirection,
        target: Box<dyn PlacedRoom>,
    ) -> Self {
        Self {
            local_shape_position,
            portal_to_room_facing,
            target,
        }
    }

    /// An immutable reference to the `OrdinalDirection` the room is facing, from the perspective of the portal.
    pub fn portal_to_room_facing(&self) -> &OrdinalDirection {
        &self.portal_to_room_facing
    }

    /// A mutable reference to the `OrdinalDirection` the room is facing, from the perspective of the portal.
    pub fn portal_to_room_facing_mut(&mut self) -> &mut OrdinalDirection {
        &mut self.portal_to_room_facing
    }

    #[allow(clippy::borrowed_box)]
    /// Returns an immutable reference to the `Box<dyn PlacedRoom>` portal target.
    pub fn target(&self) -> &Box<dyn PlacedRoom> {
        &self.target
    }

    /// Returns a mutable reference to the `Box<dyn PlacedRoom>` portal target.
    pub fn target_mut(&mut self) -> &mut Box<dyn PlacedRoom> {
        &mut self.target
    }
}

impl HasShapePosition for Portal {
    fn shape_position(&self) -> &ShapePosition {
        &self.local_shape_position
    }

    fn shape_position_mut(&mut self) -> &mut ShapePosition {
        &mut self.local_shape_position
    }
}
