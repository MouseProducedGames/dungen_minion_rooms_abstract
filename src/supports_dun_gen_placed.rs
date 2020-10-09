// External includes.

// Standard includes.

// Internal includes.
use super::PlacedRoom;

pub trait SupportsDunGenPlaced {
    #[allow(clippy::borrowed_box)]
    fn get_placed_map(&self) -> &Box<dyn PlacedRoom>;

    fn get_placed_map_mut(&mut self) -> &mut Box<dyn PlacedRoom>;
}
