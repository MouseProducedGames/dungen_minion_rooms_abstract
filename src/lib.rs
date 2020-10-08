// External includes.

// Standard includes.

// Internal includes.
mod area;
mod defines;
mod dun_gen;
mod has_local_position;
mod has_position;
mod intersects;
mod intersects_local;
mod intersects_local_pos;
mod intersects_pos;
mod local_position;
mod placed;
mod placed_object;
mod placed_room;
mod placed_shape;
mod portal;
mod portals;
mod portals_mut;
mod position;
mod room;
mod shape;
mod size;
mod sub_room;
mod sub_rooms;
mod sub_rooms_mut;
mod tile_type;

pub use area::Area;
pub use defines::{Coord, Length};
pub use dun_gen::DunGen;
pub use has_local_position::HasLocalPosition;
pub use has_position::HasPosition;
pub use intersects::Intersects;
pub use intersects_local::IntersectsLocal;
pub use intersects_local_pos::IntersectsLocalPos;
pub use intersects_pos::IntersectsPos;
pub use local_position::LocalPosition;
pub use placed::Placed;
pub use placed_object::PlacedObject;
pub use placed_room::PlacedRoom;
pub use placed_shape::PlacedShape;
pub use portal::Portal;
pub use portals::Portals;
pub use portals_mut::PortalsMut;
pub use position::Position;
pub use room::Room;
pub use shape::Shape;
pub use size::Size;
pub use sub_room::SubRoom;
pub use sub_rooms::SubRooms;
pub use sub_rooms_mut::SubRoomsMut;
pub use tile_type::TileType;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
