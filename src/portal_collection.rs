// External includes.

// Standard includes.

// Internal includes.
use super::{MapId, Portal, Portals, PortalsMut};
use crate::geometry::*;

/// The defining trait of a type that has a collection of [`Portal`](struct.Portal.html)s.
pub trait PortalCollection {
    /// Adds a `Portal` at a given `ShapePosition`, where the `Map` is facing a specific `OrdinalDirection` from the `Portal`'s perspective.
    fn add_portal(
        &mut self,
        local_position: Position,
        portal_to_room_facing: OrdinalDirection,
        portal_to_room_position: Position,
        target: MapId,
    );

    /// Gets an `Option` on an immutable reference to a `Portal`; returns None if the index is out of range.
    fn get_portal_at(&self, index: usize) -> Option<&Portal>;

    /// Gets an `Option` on a mutable reference to a `Portal`; returns None if the index is out of range.
    fn get_portal_at_mut(&mut self, index: usize) -> Option<&mut Portal>;

    /// Gets the number of `Portal`s contained in the collection.
    fn portal_count(&self) -> usize;

    /// Returns a `Portals` collection of immutable [`Portal`](struct.Portal.html) references for iteration.
    fn portals(&self) -> Portals;

    /// Returns a `PortalsMut` collection of mutable [`Portal`](struct.Portal.html) references for iteration.
    fn portals_mut(&mut self) -> PortalsMut;
}
