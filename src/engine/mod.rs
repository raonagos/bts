mod candle;
mod order;
mod position;

use crate::errors::{Error, Result};

pub use candle::*;
pub use order::*;
pub use position::*;

#[derive(Debug)]
pub struct Backtest {
    index: usize,
    orders: Vec<Order>,
    pub data: Vec<Candle>,
    pub positions: Vec<Position>,
}

impl Backtest {
    pub fn new(data: Vec<Candle>, _initial_balance: f64) -> Self {
        Self {
            data,
            index: 0,
            orders: Vec::new(),
            positions: Vec::new(),
        }
    }

    pub fn test_mut(&mut self) {}

    pub fn place_order(&mut self, order: Order) -> Result<()> {
        self.orders.push(order);
        Ok(())
    }

    fn open_position(&mut self, position: Position) {
        self.positions.push(position);
    }

    pub fn close_position(&mut self, position: &Position, _exit_price: f64) -> Result<()> {
        if let Some(pos_idx) = self.positions.iter().position(|p| p == position) {
            _ = self.positions.remove(pos_idx);
            return Ok(());
        }
        Err(Error::PositionNotFound)
    }

    fn execute_orders(&mut self) {
        let current_candle = self.data.get(self.index).cloned();
        if let Some(cc) = current_candle {
            let mut i = 0;
            while i < self.orders.len() {
                let price = self.orders[i].entry_price();
                if price >= cc.low() && price <= cc.high() {
                    let order = self.orders.remove(i);
                    self.open_position(Position(order));
                } else {
                    i += 1;
                }
            }
        }
    }

    fn execute_positions(&mut self) {}

    pub fn run<F>(&mut self, func: F)
    where
        F: Fn(&mut Self, &Candle),
    {
        use std::ops::AddAssign;

        while self.index < self.data.len() {
            let candle = &self.data[self.index].clone();
            func(self, candle);
            self.execute_orders();
            self.execute_positions();
            self.index.add_assign(1);
        }
    }

    pub fn reset(&mut self) {
        self.index = 0;
        self.orders = Vec::new();
        self.positions = Vec::new();
    }
}
