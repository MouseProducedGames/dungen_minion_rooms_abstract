// External includes.

// Standard includes.

// Internal includes.
use super::Room;
use crate::geometry::{HasLocalPosition, LocalPosition};

/// Contains information about a [`Room`](trait.Room.html) contanined within, and sub-ordinate to, another `Room`.
///
/// SubRoom contains a [`LocalPosition`](geometry/struct.LocalPosition.html), which designates where the `Room` is in comparison to the containing map, and a `Box<dyn Room>`.
#[derive(Clone)]
pub struct SubRoom {
    local: LocalPosition,
    value: Box<dyn Room>,
}

impl SubRoom {
    /// Constructs a new `SubRoom` from a `LocalPosition`
    pub fn new(local: LocalPosition, value: Box<dyn Room>) -> Self {
        Self { local, value }
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

impl HasLocalPosition for SubRoom {
    fn local(&self) -> &LocalPosition {
        &self.local
    }

    fn local_mut(&mut self) -> &mut LocalPosition {
        &mut self.local
    }
}
