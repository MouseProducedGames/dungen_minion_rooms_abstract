// External includes.

// Standard includes.

// Internal includes.
use super::{MapId, SupportsDunGen};

/// A trait for types that enact some aspect of dungeon generation on a [`Map`](trait.Map.html).
pub trait DoesDunGen {
    /// Accepts any type that implements `SupportsDunGen`. Implementations should generally then extract the `MapId` and defer to dun_gen_map.
    fn dun_gen(&self, target: &mut dyn SupportsDunGen);

    /// Modifies the map for the supplied 'MapId' using some formula or method.
    fn dun_gen_map(&self, map_id: MapId);
}
