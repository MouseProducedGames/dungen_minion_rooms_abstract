// External includes.

// Standard includes.

// Internal includes.
use super::Portal;

/// An iterator over mutable references to [`Portal`](struct.Portal.html).
///
/// The following is a basic example of abstracted functionality.
/// ```
/// # use dungen_minion_rooms_abstract::*;
/// # use dungen_minion_rooms_abstract::geometry::*;
/// # let mut portals_source = Vec::<Portal>::new();
/// let mut portals_mut = PortalsMut::new(&mut portals_source);
/// for portal_mut in portals_mut {
///     // Rotate all of the rooms at the ends of the portals 90 degrees to the right.
///     *portal_mut.portal_to_map_facing_mut() =
///         *portal_mut.portal_to_map_facing() + OrdinalRotation::Right90;
/// }
/// ```
pub struct PortalsMut<'a> {
    values: &'a mut [Portal],
}

impl<'a> PortalsMut<'a> {
    /// Creates a new immutable `Portal` iterator.
    pub fn new(values: &'a mut [Portal]) -> Self {
        Self { values }
    }
}

impl<'a> IntoIterator for PortalsMut<'a> {
    type Item = &'a mut Portal;
    type IntoIter = std::slice::IterMut<'a, Portal>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.iter_mut()
    }
}
