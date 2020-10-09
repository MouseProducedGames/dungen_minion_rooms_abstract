// External includes.

// Standard includes.

// Internal includes.
use super::{Room, SupportsDunGen};

/// A trait for types that enact some aspect of dungeon generation on a boxed [`Room`](trait.Room.html), and require data contained in `self` to do so.
pub trait DoesDunGen {
    /// Accepts any type that implements `SupportsDunGen`. Implementations should generally then extract the `Box<dyn Room>` and defer to dun_gen_map.
    fn dun_gen(&self, target: &mut dyn SupportsDunGen);

    /// Modifies the supplied map using some formula or method.
    fn dun_gen_map(&self, map: &mut Box<dyn Room>);
}

/// A trait for types that enact some aspect of dungeon generation on a boxed [`Room`](trait.Room.html), and can do so automatically, without reference to `self`.
pub trait DoesDunGenStatic {
    /// Accepts any type that implements `SupportsDunGen`. Implementations should generally then extract the `Box<dyn Room>` and defer to dun_gen_map_static.
    fn dun_gen_static(target: &mut dyn SupportsDunGen);

    /// Modifies the supplied map using some formula or method.
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
