use super::*;

fn get_data() -> Vec<Candle> {
    vec![Candle::from((0.0, 0.0, 0.0, 110.0, 0.0))]
}

fn get_long_data() -> Vec<Candle> {
    vec![
        Candle::from((90.0, 110.0, 80.0, 100.0, 1.0)),
        Candle::from((100.0, 119.0, 90.0, 110.0, 1.0)),
        Candle::from((110.0, 129.0, 100.0, 120.0, 1.0)),
    ]
}

fn get_short_data() -> Vec<Candle> {
    vec![
        Candle::from((150.0, 160.0, 131.0, 140.0, 1.0)),
        Candle::from((140.0, 150.0, 121.0, 130.0, 1.0)),
        Candle::from((130.0, 140.0, 111.0, 120.0, 1.0)),
    ]
}

#[test]
fn scenario_place_and_delete_order() {
    let data = get_data();
    let balance = 1000.0;
    let mut bt = Backtest::new(data, balance).unwrap();

    let candle = bt.next().unwrap();
    let price = candle.close(); // 110

    let order = Order::from((OrderType::Market(price), 1.0, OrderSide::Buy));
    bt.place_order(order.clone()).unwrap(); // lock amount 110

    assert!(!bt.orders.is_empty());
    assert_eq!(bt.balance(), 1000.0);
    assert_eq!(bt.total_balance(), 1000.0);
    assert_eq!(bt.free_balance().unwrap(), 890.0);

    bt.delete_order(&order).unwrap(); // unlock amount 110

    assert!(bt.orders.is_empty());
    assert_eq!(bt.balance(), 1000.0);
    assert_eq!(bt.total_balance(), 1000.0);
    assert_eq!(bt.free_balance().unwrap(), 1000.0);
}

#[test]
fn scenario_open_long_position_and_take_profit() {
    let data = get_long_data();
    let balance = 1000.0;
    let mut bt = Backtest::new(data, balance).unwrap();

    let candle = bt.next().unwrap();
    let price = candle.close();

    let take_profit = OrderType::TakeProfitAndStopLoss(price.addpercent(20.0), 0.0);
    let order = Order::from((OrderType::Market(price), take_profit, 1.0, OrderSide::Buy));
    bt.place_order(order).unwrap();

    assert!(!bt.orders.is_empty());
    assert!(bt.positions.is_empty());
    assert_eq!(bt.balance(), 1000.0);
    assert_eq!(bt.total_balance(), 1000.0);
    assert_eq!(bt.free_balance().unwrap(), 900.0);

    bt.execute_orders(&candle).unwrap();

    assert!(bt.orders.is_empty());
    assert!(!bt.positions.is_empty());
    assert_eq!(bt.balance(), 900.0);
    assert_eq!(bt.total_balance(), 900.0);
    assert_eq!(bt.free_balance().unwrap(), 900.0);

    bt.execute_positions(&candle).unwrap();
    assert!(!bt.positions.is_empty());

    // next tick
    let candle = bt.next().unwrap();
    bt.execute_positions(&candle).unwrap(); // close = 110, p&l = +10

    assert!(!bt.positions.is_empty());
    assert_eq!(bt.balance(), 900.0);
    assert_eq!(bt.total_balance(), 910.0); // balance + p&l
    assert_eq!(bt.free_balance().unwrap(), 910.0);

    // next tick
    let candle = bt.next().unwrap();
    bt.execute_positions(&candle).unwrap(); // close = 120, take profit matched

    assert!(bt.positions.is_empty());
    assert_eq!(bt.balance(), 1020.0);
    assert_eq!(bt.total_balance(), 1020.0);
    assert_eq!(bt.free_balance().unwrap(), 1020.0);
}

#[test]
fn scenario_open_long_position_and_stop_loss() {
    let data = get_short_data();
    let balance = 1000.0;
    let mut bt = Backtest::new(data, balance).unwrap();

    let candle = bt.next().unwrap();
    let price = candle.close();

    let stop_loss = OrderType::TakeProfitAndStopLoss(0.0, price - 20.0);
    let order = Order::from((OrderType::Market(price), stop_loss, 1.0, OrderSide::Buy));
    bt.place_order(order).unwrap();

    assert!(!bt.orders.is_empty());
    assert!(bt.positions.is_empty());
    assert_eq!(bt.balance(), 1000.0);
    assert_eq!(bt.total_balance(), 1000.0);
    assert_eq!(bt.free_balance().unwrap(), 860.0);

    bt.execute_orders(&candle).unwrap();

    assert!(bt.orders.is_empty());
    assert!(!bt.positions.is_empty());
    assert_eq!(bt.balance(), 860.0);
    assert_eq!(bt.total_balance(), 860.0);
    assert_eq!(bt.free_balance().unwrap(), 860.0);

    bt.execute_positions(&candle).unwrap();
    assert!(!bt.positions.is_empty());

    // next tick
    let candle = bt.next().unwrap();
    bt.execute_positions(&candle).unwrap(); // close = 130, p&l = -10

    assert!(!bt.positions.is_empty());
    assert_eq!(bt.balance(), 860.0);
    assert_eq!(bt.total_balance(), 850.0); // balance + p&l
    assert_eq!(bt.free_balance().unwrap(), 850.0);

    // next tick
    let candle = bt.next().unwrap();
    bt.execute_positions(&candle).unwrap(); // close = 120, stop loss matched

    assert!(bt.positions.is_empty());
    assert_eq!(bt.balance(), 980.0);
    assert_eq!(bt.total_balance(), 980.0);
    assert_eq!(bt.free_balance().unwrap(), 980.0);
}

#[test]
fn scenario_open_short_position_and_take_profit() {
    let data = get_short_data();
    let balance = 1000.0;
    let mut bt = Backtest::new(data, balance).unwrap();

    let candle = bt.next().unwrap();
    let price = candle.close();

    let take_profit = OrderType::TakeProfitAndStopLoss(price - 20.0, 0.0);
    let order = Order::from((OrderType::Market(price), take_profit, 1.0, OrderSide::Sell));
    bt.place_order(order).unwrap();

    assert!(!bt.orders.is_empty());
    assert!(bt.positions.is_empty());
    assert_eq!(bt.balance(), 1000.0);
    assert_eq!(bt.total_balance(), 1000.0);
    assert_eq!(bt.free_balance().unwrap(), 860.0);

    bt.execute_orders(&candle).unwrap();

    assert!(bt.orders.is_empty());
    assert!(!bt.positions.is_empty());
    assert_eq!(bt.balance(), 860.0);
    assert_eq!(bt.total_balance(), 860.0);
    assert_eq!(bt.free_balance().unwrap(), 860.0);

    bt.execute_positions(&candle).unwrap();
    assert!(!bt.positions.is_empty());

    // next tick
    let candle = bt.next().unwrap();
    bt.execute_positions(&candle).unwrap(); // close = 130, p&l = +10

    assert!(!bt.positions.is_empty());
    assert_eq!(bt.balance(), 860.0);
    assert_eq!(bt.total_balance(), 870.0); // balance + p&l
    assert_eq!(bt.free_balance().unwrap(), 870.0);

    // next tick
    let candle = bt.next().unwrap();
    bt.execute_positions(&candle).unwrap(); // close = 120, take profit matched

    assert!(bt.positions.is_empty());
    assert_eq!(bt.balance(), 1020.0);
    assert_eq!(bt.total_balance(), 1020.0);
    assert_eq!(bt.free_balance().unwrap(), 1020.0);
}

#[test]
fn scenario_open_short_position_and_stop_loss() {
    let data = get_long_data();
    let balance = 1000.0;
    let mut bt = Backtest::new(data, balance).unwrap();

    let candle = bt.next().unwrap();
    let price = candle.close();

    let stop_loss = OrderType::TakeProfitAndStopLoss(0.0, price.addpercent(20.0));
    let order = Order::from((OrderType::Market(price), stop_loss, 1.0, OrderSide::Sell));
    bt.place_order(order).unwrap();

    assert!(!bt.orders.is_empty());
    assert!(bt.positions.is_empty());
    assert_eq!(bt.balance(), 1000.0);
    assert_eq!(bt.total_balance(), 1000.0);
    assert_eq!(bt.free_balance().unwrap(), 900.0);

    bt.execute_orders(&candle).unwrap();

    assert!(bt.orders.is_empty());
    assert!(!bt.positions.is_empty());
    assert_eq!(bt.balance(), 900.0);
    assert_eq!(bt.total_balance(), 900.0);
    assert_eq!(bt.free_balance().unwrap(), 900.0);

    bt.execute_positions(&candle).unwrap();
    assert!(!bt.positions.is_empty());

    // next tick
    let candle = bt.next().unwrap();
    bt.execute_positions(&candle).unwrap(); // close = 110, p&l = -10

    assert!(!bt.positions.is_empty());
    assert_eq!(bt.balance(), 900.0);
    assert_eq!(bt.total_balance(), 890.0); // balance + p&l
    assert_eq!(bt.free_balance().unwrap(), 890.0);

    // next tick
    let candle = bt.next().unwrap();
    bt.execute_positions(&candle).unwrap(); // close = 120, stop loss matched

    assert!(bt.positions.is_empty());
    assert_eq!(bt.balance(), 980.0);
    assert_eq!(bt.total_balance(), 980.0);
    assert_eq!(bt.free_balance().unwrap(), 980.0);
}

#[test]
fn scenario_open_long_position_with_trailing_stop_profit() {
    // enter at 100
    // the high is 140 and the trailing stop is set to 10%
    // exit at 126
    let data = vec![
        Candle::from((99.0, 101.0, 98.0, 100.0, 1.0)),
        Candle::from((100.0, 110.0, 99.0, 108.0, 1.0)),
        Candle::from((108.0, 140.0, 108.0, 135.0, 1.0)),
        Candle::from((135.0, 139.9, 126.0, 130.0, 1.0)),
    ];

    let balance = 1000.0;
    let mut bt = Backtest::new(data, balance).unwrap();

    let candle = bt.next().unwrap();
    let price = candle.close();

    let trailing_stop = OrderType::TrailingStop(price, 10.0);
    let order = Order::from((OrderType::Market(price), trailing_stop, 1.0, OrderSide::Buy));
    bt.place_order(order).unwrap();
    bt.execute_orders(&candle).unwrap();

    assert!(!bt.positions.is_empty());
    assert_eq!(bt.balance(), 900.0);
    assert_eq!(bt.total_balance(), 900.0);
    assert_eq!(bt.free_balance().unwrap(), 900.0);

    bt.execute_positions(&candle).unwrap();
    assert!(!bt.positions.is_empty());

    // next tick
    let candle = bt.next().unwrap();
    bt.execute_positions(&candle).unwrap();

    assert!(!bt.positions.is_empty());
    assert_eq!(bt.balance(), 900.0);
    assert_eq!(bt.total_balance(), 908.0);
    assert_eq!(bt.free_balance().unwrap(), 908.0);

    // next tick
    let candle = bt.next().unwrap();
    bt.execute_positions(&candle).unwrap();
    assert!(!bt.positions.is_empty());
    assert_eq!(bt.balance(), 900.0);
    assert_eq!(bt.total_balance(), 935.0);
    assert_eq!(bt.free_balance().unwrap(), 935.0);

    // next tick
    let candle = bt.next().unwrap();
    bt.execute_positions(&candle).unwrap();
    assert!(bt.positions.is_empty());
    assert_eq!(bt.balance(), 1026.0);
    assert_eq!(bt.total_balance(), 1026.0);
    assert_eq!(bt.free_balance().unwrap(), 1026.0);
}

#[test]
fn scenario_open_long_position_with_trailing_stop_loss() {
    // enter at 100
    // the high is 100 and the trailing stop is set to 10%
    // exit at 90
    let data = vec![
        Candle::from((99.0, 100.0, 98.0, 100.0, 1.0)),
        Candle::from((100.0, 100.0, 90.0, 100.0, 1.0)),
    ];

    let balance = 1000.0;
    let mut bt = Backtest::new(data, balance).unwrap();

    let candle = bt.next().unwrap();
    let price = candle.close();

    let trailing_stop = OrderType::TrailingStop(price, 10.0);
    let order = Order::from((OrderType::Market(price), trailing_stop, 1.0, OrderSide::Buy));
    bt.place_order(order).unwrap();
    bt.execute_orders(&candle).unwrap();

    assert!(!bt.positions.is_empty());
    assert_eq!(bt.balance(), 900.0);
    assert_eq!(bt.total_balance(), 900.0);
    assert_eq!(bt.free_balance().unwrap(), 900.0);

    bt.execute_positions(&candle).unwrap();
    assert!(!bt.positions.is_empty());

    // next tick
    let candle = bt.next().unwrap();
    bt.execute_positions(&candle).unwrap();

    assert!(bt.positions.is_empty());
    assert_eq!(bt.balance(), 990.0);
    assert_eq!(bt.total_balance(), 990.0);
    assert_eq!(bt.free_balance().unwrap(), 990.0);
}
