// External includes.

// Standard includes.

// Internal includes.
use super::{Room, SupportsDunGen};

pub trait DoesDunGen {
    fn dun_gen(&self, target: &mut dyn SupportsDunGen);

    fn dun_gen_map(&self, map: &mut Box<dyn Room>);
}

pub trait DoesDunGenStatic {
    fn dun_gen_static(target: &mut dyn SupportsDunGen);

    fn dun_gen_map_static(map: &mut Box<dyn Room>);
}
