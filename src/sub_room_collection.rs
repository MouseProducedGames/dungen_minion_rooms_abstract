// External includes.

// Standard includes.

// Internal includes.
use super::{MapId, SubRoom, SubRooms, SubRoomsMut};
use crate::geometry::Position;

/// The defining trait of a type that has a collection of [`SubRoom`](struct.SubRoom.html)s.
pub trait SubRoomCollection {
    /// Adds a `SubRoom` at a given `Position`.
    fn add_sub_room(&mut self, local_position: Position, target: MapId);

    /// Gets an `Option` on an immutable reference to a `SubRoom`; returns None if the index is out of range.
    fn get_sub_room_at(&self, index: usize) -> Option<&SubRoom>;

    /// Gets an `Option` on a mutable reference to a `SubRoom`; returns None if the index is out of range.
    fn get_sub_room_at_mut(&mut self, index: usize) -> Option<&mut SubRoom>;

    /// Gets the number of `SubRoom`s contained in the collection.
    fn sub_room_count(&self) -> usize;

    /// Returns a `SubRooms` collection of immutable [`SubRoom`](struct.SubRoom.html) references for iteration.
    fn sub_rooms(&self) -> SubRooms;

    /// Returns a `SubRoomsMut` collection of mutable [`SubRoom`](struct.SubRoom.html) references for iteration.
    fn sub_rooms_mut(&mut self) -> SubRoomsMut;
}
