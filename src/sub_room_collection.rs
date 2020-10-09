// External includes.

// Standard includes.

// Internal includes.
use super::{Room, SubRoom};
use crate::geometry::LocalPosition;

/// The defining trait of a type that has a collection of [`SubRoom`](trait.SubRoom.html)s.
pub trait SubRoomCollection {
    /// Adds a `SubRoom` at a given [`LocalPosition`](geometry/struct.LocalPosition.html).
    fn add_sub_room(&mut self, local: LocalPosition, target: Box<dyn Room>);

    /// Gets an `Option` on an immutable reference to a `SubRoom`; returns None if the index is out of range.
    fn get_sub_room_at(&self, index: usize) -> Option<&SubRoom>;

    /// Gets an `Option` on a mutable reference to a `SubRoom`; returns None if the index is out of range.
    fn get_sub_room_at_mut(&mut self, index: usize) -> Option<&mut SubRoom>;

    /// Gets the number of `SubRoom`s contained in the collection.
    fn sub_room_count(&self) -> usize;
}
