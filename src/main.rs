mod matching_engine;
use matching_engine::orderbook::{Order, BidOrAsk, Orderbook};
use matching_engine::engine::{MatchingEngine, TradingPair};

fn main() {
    let buy_order_alpha = Order::new(BidOrAsk::Bid, 1.5);
    let buy_order_beta = Order::new(BidOrAsk::Bid, 2.5);
    let mut orderbook = Orderbook::new();
    orderbook.add_order(4.4, buy_order_alpha);
    orderbook.add_order(4.4, buy_order_beta);

    let sell_order_alpha = Order::new(BidOrAsk::Ask, 3.5);
    let sell_order_beta = Order::new(BidOrAsk::Ask, 4.5);
    orderbook.add_order(10.0, sell_order_alpha);
    orderbook.add_order(10.0, sell_order_beta);

    //println!("{:?}", orderbook);

    let mut engine = MatchingEngine::new();
    let pair = TradingPair::new("BTC".to_string(), "USD".to_string());

    engine.add_new_market(pair.clone());
    
    let buy_order = Order::new(BidOrAsk::Bid, 6.5);
    engine.place_limit_order(pair, 10.000, buy_order).unwrap();
}
