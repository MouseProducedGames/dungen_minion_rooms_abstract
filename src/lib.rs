#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

//! Defines various map and dungeon generation enums, structs, and traits (mostly traits) for the `dungen_minion` crate.
//!
//! As the purpose of this crate is to provide traits and helper types for `dungen_minion` and `dungen_minion`'s other dependent crates to consume, the gemeral usages of this crate's traits and helper types are defined here, while their specific implementations are usually defined in those other crates.

// External includes.
#[macro_use]
extern crate lazy_static;

#[allow(missing_docs)]
#[allow(missing_doc_code_examples)]
pub mod geometry {
    pub use dungen_minion_geometry::*;
}

// Standard includes.

// Internal includes.
mod does_dun_gen;
mod dummy_map;
mod map;
mod map_id;
mod portal;
mod portal_collection;
mod portals;
mod portals_mut;
mod sub_room;
mod sub_room_collection;
mod sub_rooms;
mod sub_rooms_mut;
mod supports_dun_gen;
mod tile_type;

pub use does_dun_gen::DoesDunGen;
pub(crate) use dummy_map::DummyMap;
pub use map::{get_new_map_id, Map};
pub use map_id::{register_room, MapId, MAPS};
pub use portal::Portal;
pub use portal_collection::PortalCollection;
pub use portals::Portals;
pub use portals_mut::PortalsMut;
pub use sub_room::SubRoom;
pub use sub_room_collection::SubRoomCollection;
pub use sub_rooms::SubRooms;
pub use sub_rooms_mut::SubRoomsMut;
pub use supports_dun_gen::SupportsDunGen;
pub use tile_type::TileType;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
