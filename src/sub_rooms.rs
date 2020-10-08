// External includes.

// Standard includes.

// Internal includes.
use super::SubRoom;

pub struct SubRooms<'a> {
    values: &'a [SubRoom<'a>],
}

impl<'a> SubRooms<'a> {
    pub fn new(values: &'a [SubRoom<'a>]) -> Self {
        Self { values }
    }
}

impl<'a> IntoIterator for SubRooms<'a> {
    type Item = &'a SubRoom<'a>;
    type IntoIter = std::slice::Iter<'a, SubRoom<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.iter()
    }
}
