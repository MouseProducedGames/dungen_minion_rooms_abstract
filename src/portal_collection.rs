// External includes.

// Standard includes.

// Internal includes.
use super::{PlacedRoom, Portal};
use crate::geometry::*;

pub trait PortalCollection {
    fn add_portal(
        &mut self,
        local: LocalPosition,
        portal_to_room_facing: OrdinalDirection,
        target: Box<dyn PlacedRoom>,
    );

    fn get_portal_at(&self, index: usize) -> Option<&Portal>;

    fn get_portal_at_mut(&mut self, index: usize) -> Option<&mut Portal>;

    fn portal_count(&self) -> usize;
}
