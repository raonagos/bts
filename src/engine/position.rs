type ID = u32;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[doc(alias = "ExitRule")]
#[derive(Debug, Clone)]
pub enum PositionExitRule {
    Market,
    Limit(f64),
    StopLoss(f64),
    TakeProfit(f64),
    TrailingStop(f64),
    TakeProfitAndStopLoss((f64, f64)),
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[doc(alias = "Side")]
#[derive(Debug, Clone, PartialEq)]
pub enum PositionSide {
    Long,
    Short,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Position {
    id: ID,
    quantity: f64,
    entry_price: f64,
    side: PositionSide,
    exit_rule: PositionExitRule,
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

    pub fn cost(&self) -> f64 {
        self.entry_price * self.quantity
    }

    pub fn estimate_profit(&self, exit_price: f64) -> f64 {
        match self.side {
            PositionSide::Long => (exit_price - self.entry_price) * self.quantity,
            PositionSide::Short => (self.entry_price - exit_price) * self.quantity,
        }
    }

    pub fn profit_change(&self, exit_price: f64) -> f64 {
        let mut v1 = self.entry_price * self.quantity;
        let mut v2 = exit_price * self.quantity;
        if self.side == PositionSide::Short {
            let temp = v1;
            v1 = v2;
            v2 = temp;
        }
        (v2 - v1) / v1 * 100.0
    }

    pub fn exit_rule(&self) -> &PositionExitRule {
        &self.exit_rule
    }
}

type P1 = (PositionSide, f64, f64, PositionExitRule);
impl From<P1> for Position {
    fn from((side, entry_price, quantity, exit_rule): P1) -> Self {
        Self {
            id: Self::random_id(),
            side,
            quantity,
            exit_rule,
            entry_price,
        }
    }
}

type P2 = (ID, PositionSide, f64, f64, PositionExitRule);
impl From<P2> for Position {
    fn from((id, side, entry_price, quantity, exit_rule): P2) -> Self {
        Self {
            id,
            side,
            quantity,
            exit_rule,
            entry_price,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_event() {
        let position: Position = (PositionSide::Long, 1.0, 1.0, PositionExitRule::Market).into();
        let mut event = PositionEvent::from((position.id, 1, position.side, position.entry_price));
        event.close(3, 2.0);
        assert_eq!(event.len(), 2);
    }
}
