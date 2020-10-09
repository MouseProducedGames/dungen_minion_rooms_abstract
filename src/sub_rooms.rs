// External includes.

// Standard includes.

// Internal includes.
use super::SubRoom;

pub struct SubRooms<'a> {
    values: &'a [SubRoom],
}

impl<'a> SubRooms<'a> {
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
