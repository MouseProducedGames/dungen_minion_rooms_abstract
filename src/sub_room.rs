// External includes.

// Standard includes.

// Internal includes.
use super::Room;
use crate::geometry::{HasShapePosition, ShapePosition};

/// Contains information about a [`Room`](trait.Room.html) contanined within, and sub-ordinate to, another `Room`.
///
/// SubRoom contains a [`ShapePosition`](geometry/struct.ShapePosition.html), which designates where the `Room` is in comparison to the containing map, and a `Box<dyn Room>`.
#[derive(Clone)]
pub struct SubRoom {
    local_shape_position: ShapePosition,
    value: Box<dyn Room>,
}

impl SubRoom {
    /// Constructs a new `SubRoom` from a `ShapePosition`
    pub fn new(local_shape_position: ShapePosition, value: Box<dyn Room>) -> Self {
        Self {
            local_shape_position,
            value,
        }
    }

    #[allow(clippy::borrowed_box)]
    /// Returns an immutable reference to the `Box<dyn Room>`.
    pub fn value(&self) -> &Box<dyn Room> {
        &self.value
    }

    /// Returns a mutable reference to the `Box<dyn Room>`.
    pub fn value_mut(&mut self) -> &mut Box<dyn Room> {
        &mut self.value
    }
}

impl HasShapePosition for SubRoom {
    fn shape_position(&self) -> &ShapePosition {
        &self.local_shape_position
    }

    fn shape_position_mut(&mut self) -> &mut ShapePosition {
        &mut self.local_shape_position
    }
}
