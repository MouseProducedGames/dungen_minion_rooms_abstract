// External includes.

// Standard includes.

// Internal includes.
use super::Room;
use crate::geometry::{HasLocalPosition, LocalPosition};

#[derive(Copy, Clone)]
pub struct SubRoom<'a> {
    local: LocalPosition,
    value: &'a dyn Room<'a>,
}

impl<'a> SubRoom<'a> {
    pub fn new(local: LocalPosition, value: &'a dyn Room<'a>) -> Self {
        Self { local, value }
    }
    
    pub fn value(&'a self) -> &'a dyn Room<'a> {
        self.value
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
