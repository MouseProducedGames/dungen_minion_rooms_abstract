// External includes.

// Standard includes.

// Internal includes.
use super::SubMap;

/// An iterator over immutable references to [`SubMap`](struct.SubMap.html).
///
/// The following is a basic example of abstracted functionality.
/// ```
/// # use dungen_minion_rooms_abstract::*;
/// # use dungen_minion_rooms_abstract::geometry::*;
/// # let sub_maps_source = Vec::<SubMap>::new();
/// let mut local_positions = Vec::new();
/// let sub_maps = SubMaps::new(&sub_maps_source);
/// for sub_map in sub_maps {
///     // Store the local position of each `SubMap`.
///     local_positions.push(*sub_map.local_position());
/// }
/// ```
pub struct SubMaps<'a> {
    values: &'a [SubMap],
}

impl<'a> SubMaps<'a> {
    /// Creates a new immutable `SubMap` iterator.
    #[must_use]
    pub fn new(values: &'a [SubMap]) -> Self {
        Self { values }
    }
}

impl<'a> IntoIterator for SubMaps<'a> {
    type Item = &'a SubMap;
    type IntoIter = std::slice::Iter<'a, SubMap>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.iter()
    }
}
