mod candle;
mod order;
mod position;
mod wallet;

use crate::{
    PercentCalculus,
    errors::{Error, Result},
};

pub use candle::*;
pub use order::*;
pub use position::*;
use wallet::*;

/// Backtesting engine for trading strategies.
#[derive(Debug)]
pub struct Backtest {
    index: usize,
    wallet: Wallet,
    orders: Vec<Order>,
    pub data: Vec<Candle>,
    pub positions: Vec<Position>,
}

impl Backtest {
    /// Creates a new backtest instance with the given candle data.
    pub fn new(data: Vec<Candle>, initial_balance: f64) -> Self {
        Self {
            data,
            index: 0,
            orders: Vec::new(),
            positions: Vec::new(),
            wallet: Wallet::new(initial_balance),
        }
    }

    /// Places a new order.
    pub fn place_order(&mut self, order: Order) -> Result<()> {
        let cost = order.cost();
        if !self.wallet.lock(cost) {
            return Err(Error::InsufficientFunds(cost));
        }
        self.orders.push(order);
        Ok(())
    }

    /// Opens a new position.
    fn open_position(&mut self, position: Position) {
        self.wallet.sub(position.cost());
        self.positions.push(position);
    }

    /// Closes an existing position.
    pub fn close_position(&mut self, position: &Position, exit_price: f64) -> Result<f64> {
        if let Some(pos_idx) = self.positions.iter().position(|p| p == position) {
            _ = self.positions.remove(pos_idx);
            let cost = position.cost();
            self.wallet.unlock(cost);

            // Calculate profit/loss and update wallet
            let entry_price = position.entry_price();
            let quantity = position.quantity;
            let profit = match position.side {
                PositionSide::Long => (exit_price - entry_price) * quantity,
                PositionSide::Short => (entry_price - exit_price) * quantity,
            };
            self.wallet.add(profit);

            return Ok(profit);
        }
        Err(Error::PositionNotFound)
    }

    /// Executes pending orders based on current candle data.
    fn execute_orders(&mut self) {
        let current_candle = self.data.get(self.index).cloned();
        if let Some(cc) = current_candle {
            let mut i = 0;
            while i < self.orders.len() {
                let price = self.orders[i].entry_price();
                if price >= cc.low() && price <= cc.high() {
                    let order = self.orders.remove(i);
                    self.open_position(Position::from(order));
                } else {
                    i += 1;
                }
            }
        }
    }

    /// Executes position management (take-profit, stop-loss, trailing stop).
    fn execute_positions(&mut self) {
        let current_candle = self.data.get(self.index).cloned();
        if let Some(cc) = current_candle {
            let mut i = 0;
            while i < self.positions.len() {
                let position = &self.positions[i].clone();
                let should_close = match position.exit_rule() {
                    Some(OrderType::TakeProfitAndStopLoss(take_profit, stop_loss)) => {
                        match position.side {
                            PositionSide::Long => {
                                (take_profit > &0.0 && take_profit <= &cc.high())
                                    || (stop_loss > &0.0 && stop_loss >= &cc.low())
                            }
                            PositionSide::Short => {
                                (take_profit > &0.0 && take_profit >= &cc.low())
                                    || (stop_loss > &0.0 && stop_loss <= &cc.high())
                            }
                        }
                    }
                    Some(OrderType::TrailingStop(trail_price, trail_percent)) => {
                        match position.side {
                            PositionSide::Long => {
                                let new_trailing_stop = cc.high().subpercent(*trail_percent);
                                let mut pos = self.positions[i].clone();
                                pos.set_trailingstop(new_trailing_stop);
                                self.positions[i] = pos;

                                cc.low() <= *trail_price
                            }
                            PositionSide::Short => {
                                let new_trailing_stop = cc.low().addpercent(*trail_percent);
                                let mut pos = self.positions[i].clone();
                                pos.set_trailingstop(new_trailing_stop);
                                self.positions[i] = pos;

                                cc.high() >= *trail_price
                            }
                        }
                    }
                    _ => false,
                };

                if should_close {
                    let position = self.positions.remove(i);
                    let exit_price = match position.exit_rule() {
                        Some(OrderType::TakeProfitAndStopLoss(take_profit, stop_loss)) => {
                            match position.side {
                                PositionSide::Long => {
                                    if take_profit > &0.0 && take_profit <= &cc.high() {
                                        *take_profit
                                    } else {
                                        *stop_loss
                                    }
                                }
                                PositionSide::Short => {
                                    if take_profit > &0.0 && take_profit >= &cc.low() {
                                        *take_profit
                                    } else {
                                        *stop_loss
                                    }
                                }
                            }
                        }
                        Some(OrderType::TrailingStop(price, percent)) => match position.side {
                            PositionSide::Long => price.subpercent(*percent),
                            PositionSide::Short => price.addpercent(*percent),
                        },
                        _ => unreachable!(),
                    };
                    _ = self.close_position(&position, exit_price);
                } else {
                    i += 1;
                }
            }
        }
    }

    /// Runs the backtest, executing the provided function for each candle.
    /// Throw an error if the wallet balance reaches zero.
    pub fn run<F>(&mut self, mut func: F) -> Result<()>
    where
        F: FnMut(&mut Self, &Candle),
    {
        use std::ops::AddAssign;

        while self.index < self.data.len() {
            if self.wallet.free_balance() <= 0.0 {
                return Err(Error::InsufficientFunds(0.0));
            }

            let candle = &self.data[self.index].clone();
            func(self, candle);
            self.execute_orders();
            self.execute_positions();
            self.index.add_assign(1);
        }

        Ok(())
    }

    /// Resets the backtest to its initial state.
    pub fn reset(&mut self) {
        self.index = 0;
        self.wallet.reset();
        self.orders = Vec::new();
        self.positions = Vec::new();
    }
}
