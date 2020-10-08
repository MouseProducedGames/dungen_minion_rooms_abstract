// External includes.

// Standard includes.

// Internal includes.
use super::Room;

pub struct DunGen<'a> {
    map: &'a mut dyn Room<'a>,
}

impl<'a> DunGen<'a> {
    fn build(&'a mut self) -> &'a dyn Room<'a> {
        self.map
    }
}
