// External includes.

// Standard includes.

// Internal includes.
use super::SubRoom;

pub struct SubRooms<'a> {
    values: &'a Vec<SubRoom<'a>>,
}

impl<'a> IntoIterator for SubRooms<'a> {
    type Item = SubRoom<'a>;
    type IntoIter = std::iter::Copied<std::slice::Iter<'a, Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.iter().copied()
    }
}
