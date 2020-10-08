// External includes.

// Standard includes.

// Internal includes.
use super::PlacedRoom;
use crate::geometry::{HasLocalPosition, LocalPosition};

#[derive(Copy, Clone)]
pub struct Portal<'a> {
    local: LocalPosition,
    target: &'a dyn PlacedRoom<'a>,
}

impl<'a> Portal<'a> {
    pub fn new(local: LocalPosition, target: &'a dyn PlacedRoom<'a>) -> Self {
        Self { local, target }
    }
    
    fn target(&'a self) -> &'a dyn PlacedRoom<'a> {
        self.target
    }
}

impl<'a> HasLocalPosition for Portal<'a> {
    fn local(&self) -> &LocalPosition {
        &self.local
    }

    fn local_mut(&mut self) -> &mut LocalPosition {
        &mut self.local
    }
}
