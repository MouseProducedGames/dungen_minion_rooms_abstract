// External includes.

// Standard includes.

// Internal includes.
use super::{Room, SupportsDunGen};

pub trait DoesDunGen<'a> {
    fn dun_gen(&'a self, target: &'a mut dyn SupportsDunGen<'a>);
}

pub trait DoesDunGenStatic<'a> {
    fn dun_gen_static(target: &'a mut dyn SupportsDunGen<'a>);
}
