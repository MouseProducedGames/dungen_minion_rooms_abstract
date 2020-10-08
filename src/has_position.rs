// External includes.

// Standard includes.

// Internal includes.
use super::Position;

pub trait HasPosition {
    fn pos(&self) -> &Position;
}
