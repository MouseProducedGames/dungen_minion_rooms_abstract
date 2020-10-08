// External includes.

// Standard includes.

// Internal includes.
use super::Room;

pub trait SupportsDunGen<'a> {
    fn get_map(&self) -> &dyn Room<'a>;

    fn get_map_mut(&mut self) -> &mut dyn Room<'a>;
}
