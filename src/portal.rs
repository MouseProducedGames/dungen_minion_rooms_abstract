// External includes.

// Standard includes.

// Internal includes.
use super::MapId;
use crate::geometry::*;

/// Contains information about a [`Map`](trait.Map.html) that can be reached from this `Portal`.
///
/// `Portal` contains a [`Position`](geometry/struct.Position.html), which designates where the `Portal` is on the map the `Portal` originates from, an [`OrdinalDirection`](geometry/enum.OrdinalDirection.html) which designates in which direction the target room faces from the perspective of the portal, and a `MapId` for the target room.
#[derive(Clone)]
pub struct Portal {
    local_position: Position,
    portal_to_room_facing: OrdinalDirection,
    portal_to_room_position: Position,
    target: MapId,
}

impl Portal {
    /// Creates a new `Portal` at a given `ShapePosition`, where the `Map` is facing a specific `OrdinalDirection` from the `Portal`'s perspective.
    pub fn new(
        local_position: Position,
        portal_to_room_facing: OrdinalDirection,
        portal_to_room_position: Position,
        target: MapId,
    ) -> Self {
        Self {
            local_position,
            portal_to_room_facing,
            portal_to_room_position,
            target,
        }
    }

    pub fn local_position(&self) -> &Position {
        &self.local_position
    }

    pub fn local_position_mut(&mut self) -> &mut Position {
        &mut self.local_position
    }

    /// An immutable reference to the `OrdinalDirection` the room is facing, from the perspective of the portal.
    pub fn portal_to_room_facing(&self) -> &OrdinalDirection {
        &self.portal_to_room_facing
    }

    /// A mutable reference to the `OrdinalDirection` the room is facing, from the perspective of the portal.
    pub fn portal_to_room_facing_mut(&mut self) -> &mut OrdinalDirection {
        &mut self.portal_to_room_facing
    }

    pub fn portal_to_room_position(&self) -> &Position {
        &self.portal_to_room_position
    }

    pub fn portal_to_room_position_mut(&mut self) -> &mut Position {
        &mut self.portal_to_room_position
    }

    #[allow(clippy::borrowed_box)]
    /// Returns an immutable reference to the `MapId` of the portal target.
    pub fn target(&self) -> MapId {
        self.target
    }

    /// Returns a mutable reference to the `MapId` of the portal target.
    pub fn target_mut(&mut self) -> &MapId {
        &mut self.target
    }
}
