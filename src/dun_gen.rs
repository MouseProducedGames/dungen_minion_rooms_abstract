// External includes.

// Standard includes.

// Internal includes.
use super::Room;

pub struct DunGen<'a> {
    map: &'a mut dyn Room,
}

impl<'a> DunGen<'a> {
    fn build(&'a mut self) -> &'a dyn Room {
        self.map
    }
}
