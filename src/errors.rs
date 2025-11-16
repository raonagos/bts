/// Enum representing possible errors in the crate.
pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// The candle data provided is empty.
    ///
    /// Backtesting requires at least one candle to execute.
    #[error("Candle data is empty: backtesting requires at least one candle")]
    CandleDataEmpty,

    /// The requested candle was not found in the dataset.
    #[error("Candle not found")]
    CandleNotFound,

    /// The initial or current balance is not positive.
    ///
    /// ### Arguments
    /// * `0` - The invalid balance value.
    #[error("Balance must be positive (got: {0})")]
    NegZeroBalance(f64),

    /// The wallet does not have enough funds to place the order.
    ///
    /// ### Arguments
    /// * `0` - The required amount.
    /// * `1` - The available amount.
    #[error("Insufficient funds: required {0}, available {1}")]
    InsufficientFunds(f64, f64),

    /// The free balance is negative.
    ///
    /// ### Arguments
    /// * `0` - The current balance.
    /// * `1` - The locked funds.
    #[error("Negative free balance: balance={0}, locked={1}")]
    NegFreeBalance(f64, f64),

    /// The requested order was not found.
    #[error("Order not found")]
    OrderNotFound,

    /// The requested position was not found.
    #[error("Position not found")]
    PositionNotFound,

    /// A generic error with a custom message.
    ///
    /// ### Arguments
    /// * `0` - The error message.
    #[error("{0}")]
    Msg(String),

    /// Take profit or stop loss values must be positive.
    #[error("TakeProfit or StopLoss must be positive")]
    NegTakeProfitAndStopLoss,

    /// Trailing stop values must be positive.
    #[error("TrailingStop must be positive and greater than 0")]
    NegZeroTrailingStop,

    /// The order type is not compatible with the operation.
    ///
    /// Use market or limit orders to open a position, and take profit, stop loss, or trailing stop to close a position.
    #[error("Try another order type")]
    MismatchedOrderType,

    /// An I/O error occurred.
    ///
    /// ### Arguments
    /// * `0` - The underlying I/O error.
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// A JSON serialization/deserialization error occurred.
    ///
    /// ### Arguments
    /// * `0` - The underlying JSON error.
    #[cfg(feature = "serde")]
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
}
