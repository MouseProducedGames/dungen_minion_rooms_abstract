// External includes.
use parking_lot::RwLock;

// Standard includes.

// Internal includes.
use super::{DummyMap, Map};

lazy_static! {
    /// Global Vec storage for `Map` implementations.
    pub static ref MAPS: RwLock<Vec<RwLock<Box<dyn Map>>>> = RwLock::new(Vec::new());
}

/// Call this to register a `Map`.
pub fn register_map<TMap>(map: TMap) -> MapId
where
    TMap: 'static + Map,
{
    let output = map.map_id();

    let mut maps = MAPS.write();
    if output >= maps.len() {
        maps.resize_with(output + 1, || RwLock::new(Box::new(DummyMap::new())));
    }

    maps[output] = RwLock::new(Box::new(map));

    output
}

/// A definition for `MapId`, so that the implementation can be more easily changed if needed or desired.
pub type MapId = usize;
