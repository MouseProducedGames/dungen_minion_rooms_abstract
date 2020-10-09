// External includes.

// Standard includes.

// Internal includes.
use super::Room;

pub trait SupportsDunGen {
    #[allow(clippy::borrowed_box)]
    fn get_map(&self) -> &Box<dyn Room>;

    fn get_map_mut(&mut self) -> &mut Box<dyn Room>;
}
