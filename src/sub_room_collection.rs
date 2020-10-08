// External includes.

// Standard includes.

// Internal includes.
use super::{Room, SubRoom};
use crate::geometry::LocalPosition;

pub trait SubRoomCollection<'a> {
    fn add_sub_room(&mut self, local: LocalPosition, target: &'static dyn Room<'a>);

    fn get_sub_room_at(&self, index: usize) -> Option<&SubRoom<'a>>;

    fn get_sub_room_at_mut(&mut self, index: usize) -> Option<&mut SubRoom<'a>>;

    fn sub_room_count(&self) -> usize;
}
