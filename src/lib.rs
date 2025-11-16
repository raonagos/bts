pub mod engine;
pub mod errors;
pub mod utils;

pub mod prelude {
    pub use super::*;
    pub use crate::engine::*;
    pub use crate::errors::*;
    pub use crate::utils::*;
}

use std::ops::{Add, Div, Mul, Sub};

/// Trait for performing percentage-based calculations.
///
/// This trait provides methods to add, subtract, and calculate percentages
/// for numeric types, enabling common financial calculations.
pub trait PercentCalculus<Rhs = Self> {
    /// Adds a percentage to the value.
    ///
    /// ### Arguments
    /// * `rhs` - The percentage to add (e.g., 10.0 for 10%).
    ///
    /// ### Returns
    /// The value increased by the given percentage.
    fn addpercent(self, rhs: Rhs) -> Self;

    /// Subtracts a percentage from the value.
    ///
    /// ### Arguments
    /// * `rhs` - The percentage to subtract (e.g., 10.0 for 10%).
    ///
    /// ### Returns
    /// The value decreased by the given percentage.
    fn subpercent(self, rhs: Rhs) -> Self;

    /// Calculates the absolute value of a percentage.
    ///
    /// ### Arguments
    /// * `percent` - The percentage to calculate (e.g., 10.0 for 10%).
    ///
    /// ### Returns
    /// The absolute value of the given percentage.
    fn how_many(self, percent: Self) -> Self;

    /// Calculates the percentage change between two values.
    ///
    /// ### Arguments
    /// * `new` - The new value to compare with.
    ///
    /// ### Returns
    /// The percentage change from the original value to the new value.
    fn change(self, new: Self) -> Self;
}

impl PercentCalculus for f64 {
    fn addpercent(self, percent: Self) -> Self {
        self.add(self.mul(percent.div(100.0)))
    }

    fn subpercent(self, percent: Self) -> Self {
        self.sub(self.mul(percent.div(100.0)))
    }

    fn how_many(self, percent: Self) -> Self {
        percent.mul(self.div(100.0))
    }

    fn change(self, new: Self) -> Self {
        new.sub(self).div(self).mul(100.0)
    }
}

#[cfg(test)]
mod percent {
    use super::*;

    #[test]
    fn add() {
        assert_eq!(110.0, 100.0.addpercent(10.0))
    }

    #[test]
    fn sub() {
        assert_eq!(90.0, 100.0.subpercent(10.0))
    }

    #[test]
    fn how_many() {
        assert_eq!(10.0, 100.0.how_many(10.0))
    }

    #[test]
    fn change() {
        assert_eq!(10.0, 100.0.change(110.0))
    }
}
