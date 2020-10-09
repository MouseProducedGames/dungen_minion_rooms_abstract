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

/* impl<TDoesDunGen> DoesDunGen for &TDoesDunGen
where
    TDoesDunGen: DoesDunGen
{
    fn dun_gen(&self, target: &mut dyn SupportsDunGen) {
        println!("Woah!");
        DoesDunGen::dun_gen(*self, target)
    }

    fn dun_gen_map(&self, map: &mut Box<dyn Room>) {
        println!("Oh no!");
        DoesDunGen::dun_gen_map(*self, map)
    }
}*/
