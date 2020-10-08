// External includes.

// Standard includes.

// Internal includes.
use super::PlacedRoom;
use crate::geometry::{HasLocalPosition, LocalPosition};

#[derive(Copy, Clone)]
pub struct Portal<'a> {
    local: LocalPosition,
    other: &'a dyn PlacedRoom<'a>,
}

impl<'a> Portal<'a> {
    fn other(&'a self) -> &'a dyn PlacedRoom<'a> {
        self.other
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
