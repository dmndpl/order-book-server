use std::cmp::Ordering;

use super::{order_queue::MatchResult, side::Side};

#[derive(Clone)]
pub struct Order {
    pub trader: String,
    pub side: Side,
    pub size: i32,
    pub price: i32,
}

impl Order {
    pub fn new(trader: String, side: Side, size: i32, price: i32) -> Order {
        Order {
            trader: trader,
            side: side,
            size: size,
            price: price,
        }
    }

    pub fn match_order(&mut self, order: Order) -> MatchResult {
        unimplemented!("I don't know how to match orders yet!");
    }
}

impl Ord for Order {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.price < other.price {
            match self.side {
                Side::Buy => Ordering::Less,
                Side::Sell => Ordering::Greater,
            }
        } else {
            match self.side {
                Side::Buy => Ordering::Greater,
                Side::Sell => Ordering::Less,
            }
        }
    }
}

impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price
    }
}

impl Eq for Order {}
