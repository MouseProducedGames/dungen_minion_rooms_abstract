// External includes.

// Standard includes.

// Internal includes.
use super::MapId;

/// A trait for types that support dungeon generation on a [`Map`](trait.Map.html).
pub trait SupportsDunGen {
    /// Gets a reference to a `MapId` for dungeon generation calculations.
    fn get_map_id(&self) -> MapId;
}
