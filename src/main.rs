mod matching_engine;
use matching_engine::engine::{MatchingEngine, TradingPair};
use matching_engine::orderbook::{BidOrAsk, Order, Orderbook};

use rust_decimal_macros::dec;

fn main() {
    let buy_order_alpha = Order::new(BidOrAsk::Bid, 1.5);
    let buy_order_beta = Order::new(BidOrAsk::Bid, 2.5);

    let mut orderbook = Orderbook::new();
    orderbook.add_limit_order(dec!(4.4), buy_order_alpha);
    orderbook.add_limit_order(dec!(4.4), buy_order_beta);

    let sell_order_alpha = Order::new(BidOrAsk::Ask, 3.5);
    orderbook.add_limit_order(dec!(10.0), sell_order_alpha);

    let mut engine = MatchingEngine::new();
    let pair = TradingPair::new("BTC".to_string(), "USD".to_string());
    engine.add_new_market(pair.clone());

    let buy_order = Order::new(BidOrAsk::Bid, 6.5);
    engine
        .place_limit_order(pair, dec!(10000), buy_order)
        .unwrap();
}
