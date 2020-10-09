// External includes.

// Standard includes.

// Internal includes.
use super::Portal;

pub struct Portals<'a> {
    values: &'a [Portal],
}

impl<'a> Portals<'a> {
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
