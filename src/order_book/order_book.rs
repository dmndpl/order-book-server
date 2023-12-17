use super::fill::Fill;
use super::order::Order;
use super::order_queue::{MatchResult, OrderQueue};
use super::side::Side;

pub struct OrderBook {
    bid_queue: OrderQueue,
    ask_queue: OrderQueue,
}

impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook {
            bid_queue: OrderQueue::new(Side::Buy),
            ask_queue: OrderQueue::new(Side::Sell),
        }
    }

    pub fn handle(&self, order: Order) -> Vec<Fill> {
        match order.side {
            Side::Sell => {
                let MatchResult {
                    maybe_order,
                    maybe_fills,
                } = self.bid_queue.match_order(order).clone();
                maybe_fills
            }
            Side::Buy => {
                let MatchResult {
                    maybe_order,
                    maybe_fills,
                } = self.bid_queue.match_order(order).clone();
                maybe_fills
            }
        }
    }
}
#[cfg(test)]
mod test {

    #[test]
    fn test_correct_queue_sell_order() {}

    #[test]
    fn test_correct_queue_buy_order() {}
}
