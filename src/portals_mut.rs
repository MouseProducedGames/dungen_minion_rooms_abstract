// External includes.

// Standard includes.

// Internal includes.
use super::Portal;

pub struct PortalsMut<'a> {
    values: &'a mut [Portal<'a>],
}

impl<'a> PortalsMut<'a> {
    pub fn new(values: &'a mut [Portal<'a>]) -> Self {
        Self { values }
    }
}

impl<'a> IntoIterator for PortalsMut<'a> {
    type Item = &'a mut Portal<'a>;
    type IntoIter = std::slice::IterMut<'a, Portal<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.iter_mut()
    }
}
