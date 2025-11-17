use bts::prelude::*;

use ta::{
    indicators::{
        ExponentialMovingAverage, MovingAverageConvergenceDivergence, MovingAverageConvergenceDivergenceOutput,
    },
    *,
};

fn main() -> anyhow::Result<()> {
    let items = get_data_from_file("data/btc.json".into())?;
    let candles = items
        .iter()
        .map(|d| Candle::from((d.open(), d.high(), d.low(), d.close(), d.volume(), d.bid())))
        .collect::<Vec<_>>();

    let initial_balance = 1_000.0;
    let mut bt = Backtest::new(candles.clone(), initial_balance)?;
    let mut ema = ExponentialMovingAverage::new(100)?;
    let mut macd = MovingAverageConvergenceDivergence::default();

    bt.run(|bt, candle| {
        let close = candle.close();
        let output = ema.next(close);
        let MovingAverageConvergenceDivergenceOutput { histogram, .. } = macd.next(close);

        let free_balance = bt.free_balance()?;
        let amount = free_balance.how_many(5.0);

        // 21: minimum to trade
        if free_balance > (initial_balance / 2.0) && amount > 21.0 && close > output && histogram > 0.0 {
            let quantity = amount / close;
            let order = (
                OrderType::Market(close),
                OrderType::TakeProfitAndStopLoss(close * 2.63, close.subpercent(2.0)),
                quantity,
                OrderSide::Buy,
            );
            bt.place_order(order.into())?;
        }

        Ok(())
    })?;

    let first_price = candles.first().unwrap().close();
    let last_price = candles.last().unwrap().close();

    bt.close_all_positions(last_price)?;

    let n = candles.len();
    let close_position_events = bt.events().filter(|e| matches!(e, Event::DelPosition(_))).count();
    println!("trades {close_position_events} / {n}");

    let new_balance = bt.balance();
    let new_balance_perf = initial_balance.change(new_balance);
    println!("performance {new_balance:.2} ({new_balance_perf:.2}%)");

    let buy_and_hold = (initial_balance / first_price) * last_price;
    let buy_and_hold_perf = first_price.change(last_price);
    println!("buy and hold {buy_and_hold:.2} ({buy_and_hold_perf:.2}%)");

    Ok(())
}
