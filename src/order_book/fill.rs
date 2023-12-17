use super::side::Side;

#[derive(Clone)]
pub struct Fill {
    trader: String,
    side: Side,
    size: i32,
    price: i32,
    counterpart: String,
}

impl Fill {
    pub fn new(
        trader: &String,
        side: Side,
        size: i32,
        price: i32,
        counterpart: &String,
    ) -> (Fill, Fill) {
        (
            Fill {
                trader: trader.clone(),
                side: side.clone(),
                size: size,
                price: price,
                counterpart: counterpart.clone(),
            },
            Fill {
                trader: counterpart.clone(),
                side: side.opposite().clone(),
                size: size,
                price: price,
                counterpart: trader.clone()
            },
        )
    }
}
