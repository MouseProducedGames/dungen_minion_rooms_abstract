// External includes.

// Standard includes.

// Internal includes.
use super::{HasLocalPosition, LocalPosition, Room};

#[derive(Copy, Clone)]
pub struct SubRoom<'a> {
    local: LocalPosition,
    other: &'a dyn Room<'a>,
}

impl<'a> SubRoom<'a> {
    fn other(&'a self) -> &'a dyn Room<'a> {
        self.other
    }
}

impl<'a> HasLocalPosition for SubRoom<'a> {
    fn local(&self) -> &LocalPosition {
        &self.local
    }

    fn local_mut(&mut self) -> &mut LocalPosition {
        &mut self.local
    }
}
