// External includes.

// Standard includes.

// Internal includes.
use super::Portal;

pub struct PortalsMut<'a> {
    values: &'a mut [Portal],
}

impl<'a> PortalsMut<'a> {
    pub fn new(values: &'a mut [Portal]) -> Self {
        Self { values }
    }
}

impl<'a> IntoIterator for PortalsMut<'a> {
    type Item = &'a mut Portal;
    type IntoIter = std::slice::IterMut<'a, Portal>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.iter_mut()
    }
}
