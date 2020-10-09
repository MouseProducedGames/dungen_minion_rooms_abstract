// External includes.

// Standard includes.

// Internal includes.

/// A generic set of non-specific tiles suitable as a base for further customization.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TileType {
    /// A general designation for "tiles" that aren't generally considered "part of the map".
    Void,
    /// Designates a non-traversable area that blocks line-of-sight.
    Wall,
    /// Designates a traversable area that does not block line-of-sight.
    Floor,
    /// Designates stairs, a door, a teleporter, or some other method of traveling to another location.
    Portal,
}
