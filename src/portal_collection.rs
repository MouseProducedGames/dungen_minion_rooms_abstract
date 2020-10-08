// External includes.

// Standard includes.

// Internal includes.
use super::{PlacedRoom, Portal};
use crate::geometry::LocalPosition;

pub trait PortalCollection<'a> {
    fn add_portal(&mut self, local: LocalPosition, target: &'static dyn PlacedRoom<'a>);

    fn get_portal_at(&self, index: usize) -> Option<&Portal<'a>>;

    fn get_portal_at_mut(&mut self, index: usize) -> Option<&mut Portal<'a>>;

    fn portal_count(&self) -> usize;
}
