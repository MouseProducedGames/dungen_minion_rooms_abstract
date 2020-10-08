// External includes.
pub mod geometry {
    pub use dungen_minion_geometry::*;
}

// Standard includes.

// Internal includes.
mod does_dun_gen;
mod placed_room;
mod placed_room_wrapper;
mod portal;
mod portal_collection;
mod portals;
mod portals_mut;
mod room;
mod sub_room;
mod sub_room_collection;
mod sub_rooms;
mod sub_rooms_mut;
mod supports_dun_gen;
mod tile_type;

pub use does_dun_gen::{DoesDunGen, DoesDunGenStatic};
pub use placed_room::PlacedRoom;
pub use portal::Portal;
pub use portal_collection::PortalCollection;
pub use portals::Portals;
pub use portals_mut::PortalsMut;
pub use room::Room;
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
