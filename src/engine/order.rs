#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::engine::{Position, PositionSide};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub enum OrderType {
    Market(f64),
    Limit(f64),
}

impl OrderType {
    pub(crate) fn inner(&self) -> f64 {
        match self {
            Self::Market(price) | Self::Limit(price) => price.to_owned(),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Order {
    _type: OrderType,
    quantity: f64,
    side: OrderSide,
}

impl Into<Position> for Order {
    fn into(self) -> Position {
        let side = match self.side {
            OrderSide::Buy => PositionSide::Long,
            OrderSide::Sell => PositionSide::Short,
        };
        Position::from((side, self._type.inner(), self.quantity))
    }
}

impl Order {
    pub fn entry_price(&self) -> f64 {
        self._type.inner()
    }

    pub fn quantity(&self) -> f64 {
        self.quantity
    }

    pub fn side(&self) -> &OrderSide {
        &self.side
    }

    pub(crate) fn cost(&self) -> f64 {
        self.entry_price() * self.quantity
    }
}
