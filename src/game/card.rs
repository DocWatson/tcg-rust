#[derive(Debug, Clone)]
pub struct Card {
    cost: i8
}

impl Card {
    pub fn new(cost: i8) -> Card {
        Card {
            cost: cost
        }
    }
}