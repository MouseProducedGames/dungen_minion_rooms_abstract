// External includes.

// Standard includes.
use std::cmp::{Eq, PartialEq};
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicUsize, Ordering};

// Internal includes.
use super::{MapId, PortalCollection, SubMapCollection, TileType};
use crate::geometry::*;

lazy_static! {
    static ref ATOMIC_ID: AtomicUsize = AtomicUsize::new(0);
}

/// Call this to get an ID for a new [`Map`](trait.Map.html).
pub fn get_new_map_id() -> MapId {
    // We don't actually care in what order we receive the Id; only that we get a different one.
    // "Ordering::Relaxed" is sufficient for that.
    ATOMIC_ID.fetch_add(1, Ordering::Relaxed)
}

/// The defining trait for a map..
pub trait Map: PlacedShape + PortalCollection + Send + Sync + SubMapCollection {
    /// A helper method to allow structs implementing `Map` to be `Clone`'ed.
    ///
    /// [https://users.rust-lang.org/t/solved-is-it-possible-to-clone-a-boxed-trait-object/1714/5](https://users.rust-lang.org/t/solved-is-it-possible-to-clone-a-boxed-trait-object/1714/5)
    fn box_clone(&self) -> Box<dyn Map>;

    /// Determines if the local `Position` is valid. Provides a default implementation.
    fn is_local_position_valid(&self, position: Position) -> bool {
        !(position.x() < 0
            || position.y() < 0
            || position.x() >= self.size().width() as i32
            || position.y() <= self.size().height() as i32)
    }

    /// Provides a very-likely unique u64 Id for a Map.
    fn map_id(&self) -> MapId;

    /// Gets an option for an immutable reference to the `TileType` at the given `Position`. Returns None if the `Position` is out of bounds, or there is no tile at that location.
    ///
    /// This method has a default implementation.
    fn tile_type_at(&self, position: Position) -> Option<&TileType> {
        let local_position = position - *self.position();
        if !self.is_local_position_valid(local_position) {
            None
        } else {
            self.tile_type_at_local(local_position)
        }
    }

    /// Gets an option for a mutable reference to the `TileType` at the given `Position`. Returns None if the `Position` is out of bounds, or there is no tile at that location.
    ///
    /// This method has a default implementation.
    fn tile_type_at_mut(&mut self, position: Position) -> Option<&mut TileType> {
        let local_position = position - *self.position();
        if !self.is_local_position_valid(local_position) {
            None
        } else {
            self.tile_type_at_local_mut(local_position)
        }
    }

    /// Gets an option for an immutable reference to the `TileType` at the given local `Position`. Returns None if the local `Position` is out of bounds, or there is no tile at that location.
    fn tile_type_at_local(&self, pos: Position) -> Option<&TileType>;

    /// Gets an option for a mutable reference to the `TileType` at the given local `Position`. Returns None if the `Position` is out of bounds, or there is no tile at that location.
    fn tile_type_at_local_mut(&mut self, pos: Position) -> Option<&mut TileType>;

    /// Sets the `TileType` at the given local `Position`, if it is valid, and returns the previous `TileType`, if any. The `Map` will expand to meet a local `Position` larger than its [`Size`](geometry/struct.Size.html).
    fn tile_type_at_local_set(&mut self, pos: Position, tile_type: TileType) -> Option<TileType>;
}

impl Clone for Box<dyn Map> {
    fn clone(&self) -> Box<dyn Map> {
        self.box_clone()
    }
}

impl Eq for Box<dyn Map> {}

impl Hash for Box<dyn Map> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.map_id().hash(state);
    }
}

impl PartialEq for Box<dyn Map> {
    fn eq(&self, other: &Self) -> bool {
        self.map_id() == other.map_id()
    }
}
