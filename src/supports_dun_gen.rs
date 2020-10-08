// External includes.

// Standard includes.

// Internal includes.
use super::Room;

pub trait SupportsDunGen<'a> {
    fn get_map(&'a self) -> &'a dyn Room<'a>;

    fn get_map_mut(&'a mut self) -> &'a mut dyn Room<'a>;
}
