// External includes.

// Standard includes.

// Internal includes.
use super::MapId;
use crate::geometry::*;

/// Contains information about a [`Map`](trait.Map.html) that can be reached from this `Portal`.
///
/// `Portal` contains a [`Position`](geometry/struct.Position.html), which designates where the `Portal` is on the map the `Portal` originates from, an [`OrdinalDirection`](geometry/enum.OrdinalDirection.html) which designates in which direction the target map faces from the perspective of the portal, a second `Position` which designates the end-point of the portal on the target `Map`, and a `MapId` for the target map.
#[derive(Clone)]
pub struct Portal {
    local_position: Position,
    portal_to_map_facing: OrdinalDirection,
    portal_to_map_position: Position,
    target: MapId,
}

impl Portal {
    /// Creates a new `Portal` at a given local `Position`, where the `Map` is facing a specific `OrdinalDirection` from the `Portal`'s perspective, with a specific end-point local `Position` on the target map.
    pub fn new(
        local_position: Position,
        portal_to_map_facing: OrdinalDirection,
        portal_to_map_position: Position,
        target: MapId,
    ) -> Self {
        Self {
            local_position,
            portal_to_map_facing,
            portal_to_map_position,
            target,
        }
    }

    /// An immutable reference to the `Portal`'s local position on its `Map` of origin.
    pub fn local_position(&self) -> &Position {
        &self.local_position
    }

    /// A mutable reference to the `Portal`'s local position on its `Map` of origin.
    pub fn local_position_mut(&mut self) -> &mut Position {
        &mut self.local_position
    }

    /// An immutable reference to the `OrdinalDirection` the map is facing, from the perspective of the portal.
    pub fn portal_to_map_facing(&self) -> &OrdinalDirection {
        &self.portal_to_map_facing
    }

    /// A mutable reference to the `OrdinalDirection` the map is facing, from the perspective of the portal.
    pub fn portal_to_map_facing_mut(&mut self) -> &mut OrdinalDirection {
        &mut self.portal_to_map_facing
    }

    /// An immutable reference to the `Portal`'s local end-point on its target `Map`.
    pub fn portal_to_map_position(&self) -> &Position {
        &self.portal_to_map_position
    }

    /// A mutable reference to the `Portal`'s local end-point on its target `Map`.
    pub fn portal_to_map_position_mut(&mut self) -> &mut Position {
        &mut self.portal_to_map_position
    }

    /// An immutable reference to the `MapId` of the portal target.
    pub fn target(&self) -> MapId {
        self.target
    }

    /// A mutable reference to the `MapId` of the portal target.
    pub fn target_mut(&mut self) -> &MapId {
        &mut self.target
    }
}
