# BTS TODO

## ✅ Done
- [x] Order management system
  - Place/delete orders with validation
  - Position opening/closing logic
- [x] Position management
  - Track open/closed positions
  - Calculate P&L and unrealized gains
- [x] Wallet management
  - Balance tracking with locked funds
  - Free balance calculations
- [x] Market simulation engine
  - Candle-by-candle execution
  - Event-based processing
- [x] Market fees implementation
  - Volume-based fee calculation
  - Applied on position open

## 📌 In Progress
- [ ] Add `Candle` builder for validation
- [ ] Create `Report` struct to wrap metrics (P&L, drawdown, Sharpe)
- [ ] Add methods to modify orders/positions (update SL/TP/trailing stop)

## 🚀 Road to v1.0.0

### Core Features
- [ ] WASM compilation support
- [ ] Timeframe aggregation (1H → 4H/8H/1D or 1D → 7D/1M)
- [ ] Multi-strategy parallel execution
- [ ] Strategy registry (5+ templates)
- [ ] Genetic parameter optimization

### Advanced Features
- [ ] Monte Carlo robustness testing
- [ ] Automated report generation (PDF/HTML)
- [ ] Web dashboard integration