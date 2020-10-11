// External includes.

// Standard includes.

// Internal includes.
use super::SubRoom;

/// An iterator over immutable references to [`SubRoom`](struct.SubRoom.html).
///
/// The following is a basic example of abstracted functionality.
/// ```
/// # use dungen_minion_rooms_abstract::*;
/// # use dungen_minion_rooms_abstract::geometry::*;
/// # let sub_rooms_source = Vec::<SubRoom>::new();
/// let mut shape_positions = Vec::new();
/// let sub_rooms = SubRooms::new(&sub_rooms_source);
/// for sub_room in sub_rooms {
///     // Store the shape position of each `SubRoom`.
///     shape_positions.push(*sub_room.shape_position());
/// }
/// ```
pub struct SubRooms<'a> {
    values: &'a [SubRoom],
}

impl<'a> SubRooms<'a> {
    /// Creates a new immutable `SubRoom` iterator.
    pub fn new(values: &'a [SubRoom]) -> Self {
        Self { values }
    }
}

impl<'a> IntoIterator for SubRooms<'a> {
    type Item = &'a SubRoom;
    type IntoIter = std::slice::Iter<'a, SubRoom>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.iter()
    }
}
