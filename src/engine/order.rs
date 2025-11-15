/// Represents the side of an order (buy or sell).
#[derive(Debug, Clone, PartialEq)]
pub enum OrderSide {
    Buy,
    Sell,
}

/// Represents the type of an order (market, limit, take-profit/stop-loss, trailing stop).
#[derive(Debug, Clone, PartialEq)]
pub enum OrderType {
    Market(f64),
    Limit(f64),
    TakeProfitAndStopLoss(f64, f64),
    TrailingStop(f64, f64),
}

impl OrderType {
    /// Returns the price associated with the order type (for Market and Limit orders).
    pub fn inner(&self) -> f64 {
        match self {
            Self::Market(price) | Self::Limit(price) => price.to_owned(),
            _ => unreachable!(),
        }
    }
}

/// Represents an order with entry and exit rules.
#[derive(Debug, Clone, PartialEq)]
pub struct Order {
    entry_type: OrderType,
    pub quantity: f64,
    pub side: OrderSide,
    exit_type: Option<OrderType>,
}

type O1 = (OrderType, f64, OrderSide);
type O2 = (OrderType, OrderType, f64, OrderSide);
impl From<O1> for Order {
    fn from((entry_type, quantity, side): O1) -> Self {
        Self {
            entry_type,
            quantity,
            side,
            exit_type: None,
        }
    }
}

impl From<O2> for Order {
    fn from((entry_type, exit_type, quantity, side): O2) -> Self {
        Self {
            entry_type,
            quantity,
            side,
            exit_type: Some(exit_type),
        }
    }
}

impl Order {
    /// Returns the entry price of the order.
    pub fn entry_price(&self) -> f64 {
        self.entry_type.inner()
    }

    /// Returns the total cost of the order (price * quantity).
    pub(crate) fn cost(&self) -> f64 {
        self.entry_type.inner() * self.quantity
    }

    /// Returns the entry type of the order.
    pub fn entry_type(&self) -> &OrderType {
        &self.entry_type
    }

    /// Returns the exit rule of the order, if any.
    pub fn exit_rule(&self) -> &Option<OrderType> {
        &self.exit_type
    }

    /// Updates the trailing stop price for the order.
    pub fn set_trailingstop(&mut self, new_price: f64) {
        if let Some(OrderType::TrailingStop(current_price, _)) = &mut self.exit_type {
            match self.side {
                OrderSide::Buy => {
                    if new_price > *current_price {
                        *current_price = new_price;
                    }
                }
                OrderSide::Sell => {
                    if new_price < *current_price {
                        *current_price = new_price;
                    }
                }
            }
        }
    }
}
