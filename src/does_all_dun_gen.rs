// External includes.

// Standard includes.

// Internal includes.
use super::{DoesAllInstancedDunGen, DoesAllStaticDunGen};

/// A trait for types that enact some aspect of dungeon generation, and can do so automatically, without reference to `self`.
pub trait DoesAllDunGen: DoesAllInstancedDunGen + DoesAllStaticDunGen {}
