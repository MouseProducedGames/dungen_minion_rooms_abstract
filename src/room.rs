// External includes.

// Standard includes.
use std::ops::{Index, IndexMut};

// Internal includes.
use super::{
    PortalCollection, Portals, PortalsMut, SubRoomCollection, SubRooms, SubRoomsMut, TileType,
};
use crate::geometry::{HasLocalPosition, LocalPosition, Shape};

/// The defining trait for a map that does not have a world position.
///
/// `Room` derives from [`LocalPosition`](geometry/struct.LocalPosition.html) to define its local displacement, [`PortalCollection`](trait.PortalCollection.html) to describe its connections to other rooms, [`Shape`](geometry/trait.Shape.html) to define the `Room`s ability to intersect positions, and [`SubRoomCollection`](trait.SubRoomCollection.html) to describe all rooms contained within this room.
pub trait Room: HasLocalPosition + PortalCollection + Shape + SubRoomCollection {
    /// A helper method to allow structs implementing `Room` to be `Clone`'ed.
    ///
    /// [https://users.rust-lang.org/t/solved-is-it-possible-to-clone-a-boxed-trait-object/1714/5](https://users.rust-lang.org/t/solved-is-it-possible-to-clone-a-boxed-trait-object/1714/5)
    fn box_clone(&self) -> Box<dyn Room>;

    /// Returns a `Portals` collection of immutable [`Portal`](struct.Portal.html) references for iteration.
    fn portals(&self) -> Portals;

    /// Returns a `PortalsMut` collection of mutable [`Portal`](struct.Portal.html) references for iteration.
    fn portals_mut(&mut self) -> PortalsMut;

    /// Returns a `SubRooms` collection of immutable [`SubRoom`](struct.SubRoom.html) references for iteration.
    fn sub_rooms(&self) -> SubRooms;

    /// Returns a `SubRoomsMut` collection of mutable [`SubRoom`](struct.SubRoom.html) references for iteration.
    fn sub_rooms_mut(&mut self) -> SubRoomsMut;

    /// Gets an option for an immutable reference to the `TileType` at the given `LocalPosition`. Returns None if the `LocalPosition` is out of bounds, or there is no tile at that location.
    fn tile_type_at_local(&self, pos: LocalPosition) -> Option<&TileType>;

    /// Gets an option for a mutable reference to the `TileType` at the given `LocalPosition`. Returns None if the `LocalPosition` is out of bounds, or there is no tile at that location.
    fn tile_type_at_local_mut(&mut self, pos: LocalPosition) -> Option<&mut TileType>;

    /// Sets the `TileType` at the given `LocalPosition`, and returns the previous `TileType`, if any. The `Room` will expand to meet a `LocalPosition` larger than its [`Size`](geometry/struct.Size.html).
    fn tile_type_at_local_set(
        &mut self,
        pos: LocalPosition,
        tile_type: TileType,
    ) -> Option<TileType>;
}

impl Clone for Box<dyn Room> {
    fn clone(&self) -> Box<dyn Room> {
        self.box_clone()
    }
}

impl Index<LocalPosition> for dyn Room {
    type Output = TileType;

    fn index(&self, pos: LocalPosition) -> &Self::Output {
        self.tile_type_at_local(pos).unwrap()
    }
}

impl IndexMut<LocalPosition> for dyn Room {
    fn index_mut(&mut self, pos: LocalPosition) -> &mut Self::Output {
        self.tile_type_at_local_mut(pos).unwrap()
    }
}
