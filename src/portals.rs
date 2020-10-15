// External includes.

// Standard includes.

// Internal includes.
use super::Portal;

/// An iterator over immutable references to [`Portal`](struct.Portal.html).
///
/// The following is a basic example of abstracted functionality.
/// ```
/// # use dungen_minion_rooms_abstract::*;
/// # use dungen_minion_rooms_abstract::geometry::*;
/// # let portals_source = Vec::<Portal>::new();
/// let mut local_facings = Vec::new();
/// let portals = Portals::new(&portals_source);
/// for portal in portals {
///     // Store the local facing of each `Portal` as it exits into the target map.
///     local_facings.push(*portal.portal_to_map_facing());
/// }
/// ```
pub struct Portals<'a> {
    values: &'a [Portal],
}

impl<'a> Portals<'a> {
    /// Creates a new immutable `Portal` iterator.
    pub fn new(values: &'a [Portal]) -> Self {
        Self { values }
    }
}

impl<'a> IntoIterator for Portals<'a> {
    type Item = &'a Portal;
    type IntoIter = std::slice::Iter<'a, Portal>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.iter()
    }
}
