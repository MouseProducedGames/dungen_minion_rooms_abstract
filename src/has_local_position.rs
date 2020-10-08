// External includes.

// Standard includes.

// Internal includes.
use super::LocalPosition;

pub trait HasLocalPosition {
    fn local(&self) -> &LocalPosition;
}
