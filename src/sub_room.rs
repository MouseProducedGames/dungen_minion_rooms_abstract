// External includes.

// Standard includes.

// Internal includes.
use super::Room;
use crate::geometry::{HasLocalPosition, LocalPosition};

#[derive(Clone)]
pub struct SubRoom {
    local: LocalPosition,
    value: Box<dyn Room>,
}

impl SubRoom {
    pub fn new(local: LocalPosition, value: Box<dyn Room>) -> Self {
        Self { local, value }
    }

    #[allow(clippy::borrowed_box)]
    pub fn value(&self) -> &Box<dyn Room> {
        &self.value
    }

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
