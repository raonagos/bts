#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Candle {
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
    bid: f64,
}

impl From<(f64, f64, f64, f64, f64)> for Candle {
    fn from((open, high, low, close, volume): (f64, f64, f64, f64, f64)) -> Self {
        Self {
            open,
            high,
            low,
            close,
            volume,
            bid: 0.0,
        }
    }
}

impl From<(f64, f64, f64, f64, f64, f64)> for Candle {
    fn from((open, high, low, close, volume, bid): (f64, f64, f64, f64, f64, f64)) -> Self {
        Self {
            open,
            high,
            low,
            close,
            volume,
            bid,
        }
    }
}

impl Candle {
    pub fn open(&self) -> f64 {
        self.open
    }

    pub fn high(&self) -> f64 {
        self.high
    }

    pub fn low(&self) -> f64 {
        self.low
    }

    pub fn close(&self) -> f64 {
        self.close
    }

    pub fn volume(&self) -> f64 {
        self.volume
    }

    pub fn ask(&self) -> f64 {
        self.volume - self.bid
    }

    pub fn bid(&self) -> f64 {
        self.bid
    }
}
