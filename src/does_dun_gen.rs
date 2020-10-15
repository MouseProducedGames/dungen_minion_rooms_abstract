// External includes.

// Standard includes.

// Internal includes.
use super::{MapId, SupportsDunGen};

/// A trait for types that enact some aspect of dungeon generation on a [`Map`](trait.Map.html).
pub trait DoesDunGen {
    /// Accepts any type that implements `SupportsDunGen`. The default implementation is to extract the `MapId` and defer to dun_gen_map.
    fn dun_gen(&self, target: &mut dyn SupportsDunGen) {
        let map_id = target.get_map_id();
        self.dun_gen_map(map_id)
    }

    /// Modifies the map for the supplied 'MapId'. See the documentation for specific implementations for details.
    fn dun_gen_map(&self, map_id: MapId);
}
