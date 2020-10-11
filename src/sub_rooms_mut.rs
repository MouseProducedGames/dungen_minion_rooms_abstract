// External includes.

// Standard includes.

// Internal includes.
use super::SubRoom;

/// An iterator over mutable references to [`SubRoom`](struct.SubRoom.html).
///
/// The following is a basic example of abstracted functionality.
/// ```
/// # use dungen_minion_rooms_abstract::*;
/// # use dungen_minion_rooms_abstract::geometry::*;
/// # let mut sub_rooms_source = Vec::<SubRoom>::new();
/// let mut sub_rooms_mut = SubRoomsMut::new(&mut sub_rooms_source);
/// for sub_room_mut in sub_rooms_mut {
///     // Move all of the `SubRoom`s 2 to the right, and 1 down.
///     *sub_room_mut.shape_position_mut() = *sub_room_mut.shape_position() + ShapePosition::new(2, 1);
/// }
/// ```
pub struct SubRoomsMut<'a> {
    values: &'a mut [SubRoom],
}

impl<'a> SubRoomsMut<'a> {
    /// Creates a new mutable `SubRoom` iterator.
    pub fn new(values: &'a mut [SubRoom]) -> Self {
        Self { values }
    }
}

impl<'a> IntoIterator for SubRoomsMut<'a> {
    type Item = &'a mut SubRoom;
    type IntoIter = std::slice::IterMut<'a, SubRoom>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.iter_mut()
    }
}
