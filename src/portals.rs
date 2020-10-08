// External includes.

// Standard includes.

// Internal includes.
use super::Portal;

pub struct Portals<'a> {
    values: &'a Vec<Portal<'a>>,
}

impl<'a> IntoIterator for Portals<'a> {
    type Item = Portal<'a>;
    type IntoIter = std::iter::Copied<std::slice::Iter<'a, Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.iter().copied()
    }
}
