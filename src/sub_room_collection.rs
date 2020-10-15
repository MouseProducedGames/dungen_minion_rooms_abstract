// External includes.

// Standard includes.

// Internal includes.
use super::{MapId, SubMap, SubMaps, SubMapsMut};
use crate::geometry::Position;

/// The defining trait of a type that has a collection of [`SubMap`](struct.SubMap.html)s.
pub trait SubMapCollection {
    /// Adds a `SubMap` at a given local `Position`.
    fn add_sub_room(&mut self, local_position: Position, target: MapId);

    /// Gets an `Option` on an immutable reference to a `SubMap`; returns None if the index is out of range.
    fn get_sub_room_at(&self, index: usize) -> Option<&SubMap>;

    /// Gets an `Option` on a mutable reference to a `SubMap`; returns None if the index is out of range.
    fn get_sub_room_at_mut(&mut self, index: usize) -> Option<&mut SubMap>;

    /// Gets the number of `SubMap`s contained in the collection.
    fn sub_room_count(&self) -> usize;

    /// Returns a `SubMaps` collection of immutable [`SubMap`](struct.SubMap.html) references for iteration.
    fn sub_rooms(&self) -> SubMaps;

    /// Returns a `SubMapsMut` collection of mutable [`SubMap`](struct.SubMap.html) references for iteration.
    fn sub_rooms_mut(&mut self) -> SubMapsMut;
}
