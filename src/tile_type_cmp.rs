// External includes.

// Standard includes.

// Internal includes.
use super::TileType;

/// Implements a priority-based comparison for `TileType`.
pub trait TileTypeCmp {
    /// Returns a priority value for a `TileType`.
    ///
    /// The priority value is unique to this `TileTypeCmp` implementation, and cannot be compared
    /// with the priority value returned by any other implementation.
    ///
    /// In addition, the priority value returned by `priority()` is not guaranteed to be comparable
    /// to the priority value returned by `priority_option()`.
    #[must_use]
    fn priority(value: &TileType) -> i32;

    /// Returns a priority value for an `Option<TileType>`.
    ///
    /// The priority value is unique to this `TileTypeCmp` implementation, and cannot be compared
    /// with the priority value returned by any other implementation.
    ///
    /// In addition, the priority value returned by `priority_option()` is not guaranteed to be
    /// comparable to the priority value returned by `priority()`.
    ///
    /// The default implementation of `priority_option()` assigns `None` a lower value then
    /// `TileType::Void`, and all `Some(value)` results to the same value they would have from priority(); however, implementations can override this.
    #[must_use]
    fn priority_option(value: &Option<TileType>) -> i32 {
        match value {
            None => Self::priority(&TileType::Void) - 1_000,
            Some(value) => Self::priority(value),
        }
    }

    /// Returns the `std::cmp::Ordering` of the priorities for the left and right `TileType`s.
    #[must_use]
    fn cmp(left: &TileType, right: &TileType) -> std::cmp::Ordering {
        Self::priority(left).cmp(&Self::priority(right))
    }

    /// Returns the `std::cmp::Ordering` of the priorities for the left and right `TileType`s.
    #[must_use]
    fn cmp_option(left: &Option<TileType>, right: &Option<TileType>) -> std::cmp::Ordering {
        Self::priority_option(left).cmp(&Self::priority_option(right))
    }

    /// Compares the priority of the two `TileType`s and returns the one with the greater priority.
    ///
    /// If both have equal priority, `left` is returned.
    #[must_use]
    fn return_greater<'a>(left: &'a TileType, right: &'a TileType) -> &'a TileType {
        match Self::cmp(left, right) {
            std::cmp::Ordering::Greater => right,
            std::cmp::Ordering::Equal | std::cmp::Ordering::Less => left,
        }
    }

    /// Compares the priority of the two `Option<TileType>`s and returns the one with the greater priority.
    ///
    /// If both have equal priority, `left` is returned.
    #[must_use]
    fn return_greater_option<'a>(
        left: &'a Option<TileType>,
        right: &'a Option<TileType>,
    ) -> &'a Option<TileType> {
        match Self::cmp_option(left, right) {
            std::cmp::Ordering::Greater => right,
            std::cmp::Ordering::Equal | std::cmp::Ordering::Less => left,
        }
    }

    /// Compares the priority of the two `Option<TileType>`s and returns the one with the lesser priority.
    ///
    /// If both have equal priority, `left` is returned.
    #[must_use]
    fn return_less_option<'a>(
        left: &'a Option<TileType>,
        right: &'a Option<TileType>,
    ) -> &'a Option<TileType> {
        match Self::cmp_option(left, right) {
            std::cmp::Ordering::Greater | std::cmp::Ordering::Equal => left,
            std::cmp::Ordering::Less => right,
        }
    }
}

/// The standard `TileTypeCmp` implementation sorts `TileType` and `Option<TileType>` in the
/// following order:
///
/// `TileType::Portal`,
/// `TileType::Floor`,
/// `TileType::Wall`,
/// (`None`, `TileType::Void`) => `i32::MIN`,
///
/// ```
/// use dungen_minion_rooms_abstract::*;
/// let mut values = [
///     Some(TileType::Void),
///     None,
///     Some(TileType::Wall),
///     Some(TileType::Floor),
///     Some(TileType::Portal),
/// ];
/// values.sort_by(TileTypeStandardCmp::cmp_option);
/// let test = [
///     Some(TileType::Portal),
///     Some(TileType::Floor),
///     Some(TileType::Wall),
///     None,
///     Some(TileType::Void),
/// ];
/// assert!(values == test);
/// ```
pub struct TileTypeStandardCmp;

impl TileTypeCmp for TileTypeStandardCmp {
    fn priority(value: &TileType) -> i32 {
        match value {
            TileType::Void => i32::MAX,
            TileType::Wall => 0,
            TileType::Floor => -1_000,
            TileType::Portal => -2_000,
        }
    }
}
