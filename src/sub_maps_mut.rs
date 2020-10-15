// External includes.

// Standard includes.

// Internal includes.
use super::SubMap;

/// An iterator over mutable references to [`SubMap`](struct.SubMap.html).
///
/// The following is a basic example of abstracted functionality.
/// ```
/// # use dungen_minion_rooms_abstract::*;
/// # use dungen_minion_rooms_abstract::geometry::*;
/// # let mut sub_rooms_source = Vec::<SubMap>::new();
/// let mut sub_rooms_mut = SubMapsMut::new(&mut sub_rooms_source);
/// for sub_room_mut in sub_rooms_mut {
///     // Move all of the `SubMap`s 2 to the right, and 1 down.
///     *sub_room_mut.local_position_mut() = *sub_room_mut.local_position() + Position::new(2, 1);
/// }
/// ```
pub struct SubMapsMut<'a> {
    values: &'a mut [SubMap],
}

impl<'a> SubMapsMut<'a> {
    /// Creates a new mutable `SubMap` iterator.
    pub fn new(values: &'a mut [SubMap]) -> Self {
        Self { values }
    }
}

impl<'a> IntoIterator for SubMapsMut<'a> {
    type Item = &'a mut SubMap;
    type IntoIter = std::slice::IterMut<'a, SubMap>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.iter_mut()
    }
}
