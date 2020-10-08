// External includes.

// Standard includes.

// Internal includes.
use super::Length;

pub trait Size {
    fn height(&self) -> Length;

    fn width(&self) -> Length;
}
