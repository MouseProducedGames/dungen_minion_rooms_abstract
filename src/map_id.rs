// External includes.
use parking_lot::RwLock;

// Standard includes.

// Internal includes.
use super::{DummyMap, Map};

lazy_static! {
    /// Global Vec storage for `Map` implementations.
    pub static ref MAPS: RwLock<Vec<RwLock<Box<dyn Map>>>> = RwLock::new(Vec::new());
    pub(crate) static ref VALID_MAPS: RwLock<Vec<RwLock<bool>>> = RwLock::new(Vec::new());
    pub(crate) static ref LOWEST_POSSIBLY_INVALID_MAP: RwLock<MapId> = RwLock::new(0);
}

/// Call this to register a `Map`.
pub fn register_map<TMap>(map: TMap) -> MapId
where
    TMap: 'static + Map,
{
    let output = map.map_id();

    let mut maps = MAPS.write();
    let mut valid_maps = VALID_MAPS.write();
    if output >= maps.len() {
        maps.resize_with(output + 1, || RwLock::new(Box::new(DummyMap::new())));
        valid_maps.resize_with(output + 1, || RwLock::new(false));
    }

    let mut lowest_possibly_invalid_map = LOWEST_POSSIBLY_INVALID_MAP.write();
    maps[output] = RwLock::new(Box::new(map));
    *valid_maps[output].write() = true;
    if output == *lowest_possibly_invalid_map {
        *lowest_possibly_invalid_map += 1;
    }

    output
}

/// A definition for `MapId`, so that the implementation can be more easily changed if needed or desired.
pub type MapId = usize;

/// Invalidates the specified map, allowing its `MapId` to be re-used.
///
/// This function should be used with care, and only when you are certain there are no other instances of the given `MapId`.
pub fn invalidate_map(map_id: MapId) {
    let maps = MAPS.read();
    let valid_maps = VALID_MAPS.read();
    let map = &mut maps[map_id].write();
    let valid_map = &mut valid_maps[map_id].write();
    let mut lowest_possibly_invalid_map = LOWEST_POSSIBLY_INVALID_MAP.write();
    **map = Box::new(DummyMap::new());
    **valid_map = false;
    if map_id < *lowest_possibly_invalid_map {
        *lowest_possibly_invalid_map = map_id;
    }
}
