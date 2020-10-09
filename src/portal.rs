// External includes.

// Standard includes.

// Internal includes.
use super::PlacedRoom;
use crate::geometry::{HasLocalPosition, LocalPosition};

#[derive(Clone)]
pub struct Portal {
    local: LocalPosition,
    target: Box<dyn PlacedRoom>,
}

impl Portal {
    pub fn new(local: LocalPosition, target: Box<dyn PlacedRoom>) -> Self {
        Self { local, target }
    }

    #[allow(clippy::borrowed_box)]
    pub fn target(&self) -> &Box<dyn PlacedRoom> {
        &self.target
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
