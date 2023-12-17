use std::collections::BinaryHeap;

use super::fill::Fill;
use super::order::Order;
use super::side::Side;

const CAPACITY: i32 = 10000;

pub struct OrderQueue {
    pub queue: BinaryHeap<Order>
}

#[derive(Clone)]
pub struct MatchResult {
    pub maybe_order: Option<Order>,
    pub maybe_fills: Vec<Fill>
}

impl OrderQueue {
    pub fn new(side: Side) -> OrderQueue {
        OrderQueue {
            queue: BinaryHeap::new()
        }
    }

    pub fn match_order(&self, order: Order) -> MatchResult {
        MatchResult { maybe_order: None, maybe_fills: Vec::new() }
    }
}