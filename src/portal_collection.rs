// External includes.

// Standard includes.

// Internal includes.
use super::{PlacedRoom, Portal};
use crate::geometry::*;

/// The defining trait of a type that has a collection of [`Portal`](struct.Portal.html)s.
pub trait PortalCollection {
    /// Adds a `Portal` at a given `LocalPosition`, where the `PlacedRoom` is facing a specific `OrdinalDirection` from the `Portal`'s perspective.
    fn add_portal(
        &mut self,
        local: LocalPosition,
        portal_to_room_facing: OrdinalDirection,
        target: Box<dyn PlacedRoom>,
    );

    /// Gets an `Option` on an immutable reference to a `Portal`; returns None if the index is out of range.
    fn get_portal_at(&self, index: usize) -> Option<&Portal>;

    /// Gets an `Option` on a mutable reference to a `Portal`; returns None if the index is out of range.
    fn get_portal_at_mut(&mut self, index: usize) -> Option<&mut Portal>;

    /// Gets the number of `Portal`s contained in the collection.
    fn portal_count(&self) -> usize;
}
