use bts::prelude::*;

fn main() -> anyhow::Result<()> {
    let items: Vec<Data> = vec![];
    let candles = items
        .iter()
        .map(|d| Candle::from((d.open(), d.high(), d.low(), d.close(), d.volume(), d.bid())))
        .collect::<Vec<_>>();

    let mut bt = Backtest::new(candles.clone(), 1_000.0);

    bt.run(|bt, _candle| {
        bt.test_mut();
    });

    Ok(())
}
