/// Represents a trading wallet with balance and locked funds management.
#[derive(Debug)]
pub(crate) struct Wallet {
    // Initial balance used for reset
    _balance: f64,
    // Available balance
    balance: f64,
    // Funds locked in open positions
    locked: f64,
}

impl Wallet {
    /// Creates a new wallet with the given initial balance.
    /// Negative balances are set to 0.
    pub fn new(balance: f64) -> Self {
        Self {
            _balance: balance.max(0.0),
            balance: balance.max(0.0),
            locked: 0.0,
        }
    }

    /// Returns the free balance (available for new trades).
    pub fn free_balance(&self) -> f64 {
        self.balance - self.locked
    }

    /// Adds funds to the wallet.
    pub fn add(&mut self, amount: f64) {
        self.balance += amount.max(0.0);
    }

    /// Subtracts funds from the wallet.
    /// Returns true if successful, false if insufficient funds.
    pub fn sub(&mut self, amount: f64) -> bool {
        if self.free_balance() >= amount {
            self.balance -= amount;
            true
        } else {
            false
        }
    }

    /// Locks additional funds for a position.
    pub fn lock(&mut self, amount: f64) -> bool {
        if self.free_balance() >= amount {
            self.locked += amount;
            true
        } else {
            false
        }
    }

    /// Unlocks funds when a position is closed.
    pub fn unlock(&mut self, amount: f64) {
        self.locked -= amount.min(self.locked);
    }

    /// Resets the wallet to its initial balance.
    pub fn reset(&mut self) {
        self.locked = 0.0;
        self.balance = self._balance;
    }
}
