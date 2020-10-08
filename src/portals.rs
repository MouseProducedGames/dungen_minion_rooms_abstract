// External includes.

// Standard includes.

// Internal includes.
use super::Portal;

pub struct Portals<'a> {
    values: &'a [Portal<'a>],
}

impl<'a> Portals<'a> {
    pub fn new(values: &'a [Portal<'a>]) -> Self {
        Self { values }
    }
}

impl<'a> IntoIterator for Portals<'a> {
    type Item = &'a Portal<'a>;
    type IntoIter = std::slice::Iter<'a, Portal<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.iter()
    }
}
