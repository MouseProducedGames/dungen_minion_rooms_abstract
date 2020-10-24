// External includes.

// Standard includes.

// Internal includes.
use super::*;
use crate::geometry::*;

/// A [`Map`](trait.Map.html) implementation intended for testing, and as a placeholder.
#[derive(Clone)]
pub(crate) struct DummyMap {
    area: Area,
}

impl DummyMap {
    pub(crate) fn new() -> Self {
        Self {
            area: Area::new(Position::zero(), Size::zero()),
        }
    }
}

impl ContainsLocalPosition for DummyMap {
    fn contains_local_position(&self, _position: Position) -> Containment {
        Containment::Disjoint
    }
}

impl ContainsPosition for DummyMap {}

impl HasArea for DummyMap {
    fn area(&self) -> &Area {
        &self.area
    }

    fn area_mut(&mut self) -> &mut Area {
        &mut self.area
    }
}

impl HasHeight for DummyMap {
    fn height(&self) -> Length {
        self.size().height()
    }

    fn height_mut(&mut self) -> &mut Length {
        self.size_mut().height_mut()
    }
}

impl HasPosition for DummyMap {
    fn position(&self) -> &Position {
        self.area.position()
    }

    fn position_mut(&mut self) -> &mut Position {
        self.area.position_mut()
    }
}

impl HasSize for DummyMap {
    fn size(&self) -> &Size {
        self.area.size()
    }

    fn size_mut(&mut self) -> &mut Size {
        self.area.size_mut()
    }
}

impl HasWidth for DummyMap {
    fn width(&self) -> Length {
        self.size().width()
    }

    fn width_mut(&mut self) -> &mut Length {
        self.size_mut().width_mut()
    }
}

impl IntersectsLocalPosition for DummyMap {
    fn intersects_local_position(&self, _position: Position) -> bool {
        false
    }
}

impl IntersectsPosition for DummyMap {}

impl Map for DummyMap {
    fn box_clone(&self) -> Box<dyn Map> {
        Box::new((*self).clone())
    }

    fn map_id(&self) -> MapId {
        0
    }

    fn rotate(&mut self, _rotation: CardinalRotation) {}

    fn tile_type_at_local(&self, _pos: Position) -> Option<TileType> {
        None
    }

    fn tile_type_at_local_mut(&mut self, _pos: Position) -> Option<&mut TileType> {
        None
    }

    fn tile_type_at_local_set(&mut self, _pos: Position, _tile_type: TileType) -> Option<TileType> {
        None
    }

    fn tile_type_at_local_sort_by<'a>(
        &self,
        _pos: Position,
        _sort_best: &dyn Fn(&Option<TileType>, &Option<TileType>) -> std::cmp::Ordering,
    ) -> Option<TileType> {
        None
    }
}

impl Placed for DummyMap {}

impl PlacedObject for DummyMap {}

impl PortalCollection for DummyMap {
    fn add_portal(
        &mut self,
        _local_position: Position,
        _portal_to_map_facing: CardinalDirection,
        _portal_to_map_position: Position,
        _target: MapId,
    ) {
    }

    fn get_portal_at(&self, _index: usize) -> Option<&Portal> {
        None
    }

    fn get_portal_at_mut(&mut self, _index: usize) -> Option<&mut Portal> {
        None
    }

    fn portal_count(&self) -> usize {
        0
    }

    fn portals(&self) -> Portals {
        Portals::new(&[])
    }

    fn portals_mut(&mut self) -> PortalsMut {
        PortalsMut::new(&mut [])
    }
}

impl Shape for DummyMap {}

impl SubMapCollection for DummyMap {
    fn add_sub_map(&mut self, _local_position: Position, _target: MapId) {}

    fn get_sub_map_at(&self, _index: usize) -> Option<&SubMap> {
        None
    }

    fn get_sub_map_at_mut(&mut self, _index: usize) -> Option<&mut SubMap> {
        None
    }

    fn sub_map_count(&self) -> usize {
        0
    }

    fn sub_maps(&self) -> SubMaps {
        SubMaps::new(&[])
    }

    fn sub_maps_mut(&mut self) -> SubMapsMut {
        SubMapsMut::new(&mut [])
    }
}
