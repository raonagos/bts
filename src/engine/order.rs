#[derive(Debug, Clone, PartialEq)]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OrderType {
    Market(f64),
    Limit(f64),
    TakeProfitAndStopLoss(f64, f64),
    TrailingStop(f64, f64),
}

impl OrderType {
    pub fn inner(&self) -> f64 {
        match self {
            Self::Market(price) | Self::Limit(price) => price.to_owned(),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Order {
    entry_type: OrderType,
    pub quantity: f64,
    pub side: OrderSide,
    exit_type: Option<OrderType>,
}

impl Order {
    pub fn entry_price(&self) -> f64 {
        self.entry_type.inner()
    }

    #[allow(unused)]
    pub(crate) fn cost(&self) -> f64 {
        self.entry_type.inner() * self.quantity
    }

    pub fn type_(&self) -> &OrderType {
        &self.entry_type
    }
}
