// External includes.

// Standard includes.

// Internal includes.
use super::Room;

/// A trait for types that support dungeon generation on a boxed [`Room`](trait.Room.html).
pub trait SupportsDunGen {
    #[allow(clippy::borrowed_box)]
    /// Gets a reference to a boxed `Room` for dungeon generation calculations that do not currently require modifying the `Room`.
    fn get_map(&self) -> &Box<dyn Room>;

    /// Gets a mutable reference to a boxed `Room`, for modification of that `Room`.
    fn get_map_mut(&mut self) -> &mut Box<dyn Room>;
}
