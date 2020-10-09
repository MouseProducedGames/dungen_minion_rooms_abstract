// External includes.

// Standard includes.

// Internal includes.
use super::{PlacedRoom, SupportsDunGenPlaced};

pub trait DoesDunGenPlaced {
    fn dun_gen_placed(&self, target: &mut dyn SupportsDunGenPlaced);

    fn dun_gen_placed_map(&self, map: &mut Box<dyn PlacedRoom>);
}

pub trait DoesDunGenPlacedStatic {
    fn dun_gen_placed_static(target: &mut dyn SupportsDunGenPlaced);

    fn dun_gen_placed_map_static(map: &mut Box<dyn PlacedRoom>);
}