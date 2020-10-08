// External includes.

// Standard includes.

// Internal includes.
use super::{HasLocalPosition, LocalPosition, Room};

#[derive(Copy, Clone)]
pub struct Portal<'a> {
    local: LocalPosition,
    other: &'a dyn Room,
}

impl<'a> Portal<'a> {
    fn other(&'a self) -> &'a dyn Room {
        self.other
    }
}

impl<'a> HasLocalPosition for Portal<'a> {
    fn local(&self) -> &LocalPosition {
        &self.local
    }
}
