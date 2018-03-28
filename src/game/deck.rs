extern crate rand;
use self::rand::{thread_rng, Rng};

use game::card::Card;

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>
}

impl Deck {
    /// Create a new Deck object
    pub fn new() -> Deck {
        let mut cards: Vec<Card> = vec![];
        // counts as defined by the Kata rules
        let mana_counts = [0,0,1,1,2,2,2,3,3,3,3,4,4,4,5,5,6,6,7,8];
        for mana in mana_counts.iter() {
            let card = Card::new(*mana);
            cards.push(card);
        }

        Deck {
            cards: cards
        }
    }

    /// Shufles the deck
    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        rng.shuffle(&mut self.cards);
    }

    /// Draws a card
    /// Since this is a mutable reference, the deck is updated when pop is called
    /// Typically, the returned card will go to the player's hand
    pub fn draw(&mut self) -> Option<Card> {
        let drawn_card = self.cards.pop();
        drawn_card
    }
}