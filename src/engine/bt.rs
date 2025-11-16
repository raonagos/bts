use super::*;

fn get_data() -> Vec<Candle> {
    vec![
        Candle::from((100.0, 111.0, 99.0, 110.0, 1.0)),
        Candle::from((110.0, 112.0, 100.0, 120.0, 1.0)),
        Candle::from((120.0, 121.0, 100.0, 110.0, 1.0)),
    ]
}

#[test]
fn long_position() {
    let data = get_data();
    let balance = 1000.0;
    let mut bt = Backtest::new(data, balance).unwrap();

    let candle = bt.next().unwrap();
    let price = candle.close();

    let order: Order = (OrderType::Market(price), 1.0, OrderSide::Buy).into();
    let result = bt.open_position(Position::from(order));
    assert!(result.is_ok());
    assert!(!bt.positions.is_empty());
    assert_eq!(bt.balance(), 1000.0 - (110.0 * 1.0)); // 890.0

    let candle = bt.next().unwrap();
    let price = candle.close();

    let position = bt.positions.front().unwrap().clone();
    let result = bt.close_position(&position, price, true);
    assert!(result.is_ok());
    let profit = result.unwrap();
    assert!(bt.positions.is_empty());
    assert_eq!(profit, (120.0 - 110.0) * 1.0); // 10.0
    assert_eq!(bt.balance(), 1010.0); // 890.0 + 110.0 + 10.0 = 1000.0 + 10.0 = 1010.0
}

#[test]
fn short_position() {
    let data = get_data();
    let balance = 1000.0;
    let mut bt = Backtest::new(data, balance).unwrap();

    bt.next().unwrap();
    let candle = bt.next().unwrap();
    let price = candle.close();

    let order: Order = (OrderType::Market(price), 1.0, OrderSide::Sell).into();
    let result = bt.open_position(Position::from(order));
    assert!(result.is_ok());
    assert!(!bt.positions.is_empty());
    assert_eq!(bt.balance(), 1000.0 - (120.0 * 1.0)); // 880.0

    let candle = bt.next().unwrap();
    let price = candle.close();

    let position = bt.positions.front().unwrap().clone();
    let result = bt.close_position(&position, price, true);
    assert!(result.is_ok());
    let profit = result.unwrap();
    assert!(bt.positions.is_empty());
    assert_eq!(profit, (120.0 - 110.0) * 1.0); // 10.0
    assert_eq!(bt.balance(), 1010.0); // 880.0 + 120.0 + 10.0 = 1010.0
}

#[test]
fn failed_long_position() {
    let data = get_data();
    let balance = 1000.0;
    let mut bt = Backtest::new(data, balance).unwrap();

    bt.next().unwrap();
    let candle = bt.next().unwrap();
    let price = candle.close();

    let order: Order = (OrderType::Market(price), 1.0, OrderSide::Buy).into();
    let result = bt.open_position(Position::from(order));
    assert!(result.is_ok());
    assert!(!bt.positions.is_empty());
    assert_eq!(bt.balance(), 1000.0 - (120.0 * 1.0)); // 880.0

    let candle = bt.next().unwrap();
    let price = candle.close();

    let position = bt.positions.front().unwrap().clone();
    let result = bt.close_position(&position, price, true);
    assert!(result.is_ok());
    let profit = result.unwrap();
    assert!(bt.positions.is_empty());
    assert_eq!(profit, (110.0 - 120.0) * 1.0); // -10.0
    assert_eq!(bt.balance(), 990.0); // 880.0 + 120.0 - 10.0 = 990.0
}

#[test]
fn failed_short_position() {
    let data = get_data();
    let balance = 1000.0;
    let mut bt = Backtest::new(data, balance).unwrap();

    let candle = bt.next().unwrap();
    let price = candle.close();

    let order: Order = (OrderType::Market(price), 1.0, OrderSide::Sell).into();
    let result = bt.open_position(Position::from(order));
    assert!(result.is_ok());
    assert!(!bt.positions.is_empty());
    assert_eq!(bt.balance(), 1000.0 - (110.0 * 1.0)); // 890.0

    let candle = bt.next().unwrap();
    let price = candle.close();

    let position = bt.positions.front().unwrap().clone();
    let result = bt.close_position(&position, price, true);
    assert!(result.is_ok());
    let profit = result.unwrap();
    assert!(bt.positions.is_empty());
    assert_eq!(profit, (110.0 - 120.0) * 1.0); // -10.0
    assert_eq!(bt.balance(), 990.0); // 890.0 + 110.0 - 10.0 = 990.0
}
