// External includes.

// Standard includes.

// Internal includes.
use super::MapId;
use crate::geometry::Position;

/// Contains information about a [`Room`](trait.Room.html) contanined within, and sub-ordinate to, another `Room`.
///
/// SubRoom contains a [`ShapePosition`](geometry/struct.ShapePosition.html), which designates where the `Room` is in comparison to the containing map, and a `MapId` for the sub-room.
#[derive(Clone)]
pub struct SubRoom {
    local_position: Position,
    value: MapId,
}

impl SubRoom {
    /// Constructs a new `SubRoom` from a `ShapePosition`
    pub fn new(local_position: Position, value: MapId) -> Self {
        Self {
            local_position,
            value,
        }
    }

    pub fn local_position(&self) -> &Position {
        &self.local_position
    }

    pub fn local_position_mut(&mut self) -> &mut Position {
        &mut self.local_position
    }

    #[allow(clippy::borrowed_box)]
    /// Returns an immutable reference to the `Box<dyn Room>`.
    pub fn value(&self) -> MapId {
        self.value
    }

    /// Returns a mutable reference to the `Box<dyn Room>`.
    pub fn value_mut(&mut self) -> &MapId {
        &mut self.value
    }
}
