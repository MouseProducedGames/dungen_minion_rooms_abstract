// External includes.

// Standard includes.

// Internal includes.
use super::MapId;
use crate::geometry::Position;

/// Contains information about a [`Map`](trait.Map.html) contanined within, and sub-ordinate to, another `Map`.
///
/// SubMap contains a local [`Position`](geometry/struct.ShapePosition.html), which designates where the sub-`Map` is in comparison to the containing `Map`, and a `MapId` for the sub-map.
#[derive(Clone)]
pub struct SubMap {
    local_position: Position,
    value: MapId,
}

impl SubMap {
    /// Constructs a new `SubMap` from a local `Position` and a `MapId` for the sub-map.
    pub fn new(local_position: Position, value: MapId) -> Self {
        Self {
            local_position,
            value,
        }
    }

    /// An immutable reference to the position of the sub-map in relation to the containing map.
    pub fn local_position(&self) -> &Position {
        &self.local_position
    }

    /// A mutable reference to the position of the sub-map in relation to the containing map.
    pub fn local_position_mut(&mut self) -> &mut Position {
        &mut self.local_position
    }

    #[allow(clippy::borrowed_box)]
    /// Returns the `MapId` for the sub-map.
    pub fn value(&self) -> MapId {
        self.value
    }

    /// A mutable reference to the `MapId` for the sub-map.
    pub fn value_mut(&mut self) -> &MapId {
        &mut self.value
    }
}
