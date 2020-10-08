// External includes.

// Standard includes.

// Internal includes.
use super::SubRoom;

pub struct SubRoomsMut<'a> {
    values: &'a mut [SubRoom<'a>],
}

impl<'a> SubRoomsMut<'a> {
    pub fn new(values: &'a mut [SubRoom<'a>]) -> Self {
        Self { values }
    }
}

impl<'a> IntoIterator for SubRoomsMut<'a> {
    type Item = &'a mut SubRoom<'a>;
    type IntoIter = std::slice::IterMut<'a, SubRoom<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.iter_mut()
    }
}
