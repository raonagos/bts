type ID = u32;

use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq)]
pub enum PositionSide {
    Long,
    Short,
}

impl fmt::Display for PositionSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PositionSide::Long => write!(f, "LONG"),
            PositionSide::Short => write!(f, "SHORT"),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Position {
    id: ID,
    side: PositionSide,
    entry_price: f64,
    quantity: f64,
}

impl Position {
    pub fn random_id() -> ID {
        use rand::Rng;

        let mut rng = rand::rng();
        rng.random_range(1..1000)
    }

    pub fn id(&self) -> ID {
        self.id
    }

    pub fn side(&self) -> PositionSide {
        self.side.clone()
    }

    pub fn quantity(&self) -> f64 {
        self.quantity
    }

    pub fn entry_price(&self) -> f64 {
        self.entry_price
    }

    pub fn estimate_profit(&self, exit_price: f64) -> f64 {
        match self.side {
            PositionSide::Long => (exit_price - self.entry_price) * self.quantity,
            PositionSide::Short => (self.entry_price - exit_price) * self.quantity,
        }
    }
}

impl From<(PositionSide, f64, f64)> for Position {
    fn from((side, entry_price, quantity): (PositionSide, f64, f64)) -> Self {
        Self {
            id: Self::random_id(),
            side,
            entry_price,
            quantity,
        }
    }
}

impl From<(ID, PositionSide, f64, f64)> for Position {
    fn from((id, side, entry_price, quantity): (ID, PositionSide, f64, f64)) -> Self {
        Self {
            id,
            side,
            entry_price,
            quantity,
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[doc(alias = "Event")]
#[derive(Debug, Clone)]
pub struct PositionEvent {
    id: u32,
    open: (usize, PositionSide, f64),
    close: Option<(usize, f64)>,
}

impl PositionEvent {
    pub fn id(&self) -> ID {
        self.id
    }

    pub fn len(&self) -> usize {
        self.close
            .map(|(pos_idx, _)| pos_idx - self.open.0)
            .unwrap_or_default()
    }

    pub fn close(&mut self, pos_idx: usize, price: f64) {
        self.close = Some((pos_idx, price));
    }
}

impl From<(ID, usize, PositionSide, f64)> for PositionEvent {
    fn from((id, pos_idx, side, price): (ID, usize, PositionSide, f64)) -> Self {
        Self {
            id,
            open: (pos_idx, side, price),
            close: None,
        }
    }
}

impl fmt::Display for PositionEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some((_, price)) = self.close {
            return write!(
                f,
                "Event(id {} len {} opened {} at {:.2} and closed at {:.2})",
                self.id,
                self.len(),
                self.open.1,
                self.open.2,
                price,
            );
        }

        write!(f, "Event(id {} opened at {:.2})", self.id, self.open.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_event() {
        let position: Position = (PositionSide::Long, 1.0, 1.0).into();
        let mut event = PositionEvent::from((position.id, 1, position.side, position.entry_price));
        event.close(3, 2.0);
        assert_eq!(event.len(), 2);
    }
}
