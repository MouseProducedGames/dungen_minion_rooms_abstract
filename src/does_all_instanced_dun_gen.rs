// External includes.

// Standard includes.

// Internal includes.
use super::{DoesDunGen, DoesDunGenPlaced};

/// A trait for types that enact some aspect of dungeon generation, and require data contained in `self` to do so.
pub trait DoesAllInstancedDunGen: DoesDunGen + DoesDunGenPlaced {}
