// External includes.

// Standard includes.

// Internal includes.
use super::PlacedRoom;

/// A trait for types that support dungeon generation on a boxed [`PlacedRoom`](trait.PlacedRoom.html).
pub trait SupportsDunGenPlaced {
    #[allow(clippy::borrowed_box)]
    /// Gets a reference to a boxed `PlacedRoom` for dungeon generation calculations that do not currently require modifying the `PlacedRoom`.
    fn get_placed_map(&self) -> &Box<dyn PlacedRoom>;

    /// Gets a mutable reference to a boxed `PlacedRoom`, for modification of that `PlacedRoom`.
    fn get_placed_map_mut(&mut self) -> &mut Box<dyn PlacedRoom>;
}
