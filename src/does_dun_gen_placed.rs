// External includes.

// Standard includes.

// Internal includes.
use super::{PlacedRoom, SupportsDunGenPlaced};

/// A trait for types that enact some aspect of dungeon generation on a boxed [`PlacedRoom`](trait.PlacedRoom.html), and require data contained in `self` to do so.
pub trait DoesDunGenPlaced {
    /// Accepts any type that implements `SupportsDunGenPlaced`. Implementations should generally then extract the `Box<dyn PlacedRoom>` and defer to dun_gen_placed_map.
    fn dun_gen_placed(&self, target: &mut dyn SupportsDunGenPlaced);

    /// Modifies the supplied map using some formula or method.
    fn dun_gen_placed_map(&self, map: &mut Box<dyn PlacedRoom>);
}

/// A trait for types that enact some aspect of dungeon generation on a boxed [`PlacedRoom`](trait.PlacedRoom.html), and can do so automatically, without reference to `self`.
pub trait DoesDunGenPlacedStatic {
    /// Accepts any type that implements `SupportsDunGenPlaced`. Implementations should generally then extract the `Box<dyn PlacedRoom>` and defer to dun_gen_placed_map_static.
    fn dun_gen_placed_static(target: &mut dyn SupportsDunGenPlaced);

    /// Modifies the supplied map using some formula or method.
    fn dun_gen_placed_map_static(map: &mut Box<dyn PlacedRoom>);
}

/* impl<TDoesDunGenPlaced> DoesDunGenPlaced for &TDoesDunGenPlaced
where
    TDoesDunGenPlaced: DoesDunGenPlaced
{
    fn dun_gen_placed(&self, target: &mut dyn SupportsDunGenPlaced) {
        println!("Woah!");
        DoesDunGenPlaced::dun_gen_placed(*self, target)
    }

    fn dun_gen_placed_map(&self, map: &mut Box<dyn PlacedRoom>) {
        println!("Oh no!");
        DoesDunGenPlaced::dun_gen_placed_map(*self, map)
    }
} */
