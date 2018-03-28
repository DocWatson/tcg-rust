use game::card::Card;
use game::deck::Deck;
use game::mana::Mana;

#[derive(Debug)]
pub struct Player {
    /// The Player's Name
    name: String,
    /// The Player's remaining health
    health: i8,
    /// The Player's Mana
    mana: Mana,
    /// The player's hand
    hand: Vec<Card>,
    /// The player's remaining deck
    deck: Deck,
    /// The graveyard (played cards)
    graveyard: Vec<Card>
}

impl Player {
    pub fn new(name: String) -> Player {
        let mut deck = Deck::new();
        let mut initial_hand = vec![];
        deck.shuffle();

        // Create the initial hand by drawing 3 cards
        for _draw in 0..3 {
            let drawn_card = deck.draw();
            // since this is the initial draw, it's safe to assume the
            // drawn cards are not `None`; a safer version of draw is below
            initial_hand.push(drawn_card.unwrap());
        }

        Player {
            name: name,
            health: 30,
            mana: Mana::new(0, 0),
            hand: initial_hand,
            deck: deck,
            graveyard: vec![]
        }
    }

    pub fn draw(&mut self) {
        let mut current_hand = self.hand.clone();
        let drawn_card = self.deck.draw();

        // the draw method will return None if the deck is empty
        if drawn_card.is_none() {
            // if the deck is in fact empty, take 1 damage instead
            self.take_damage(1);
        } else {
            let card = drawn_card.unwrap();
            // The hand is full at five cards
            // Immediately discard the drawn card if that is the case
            if current_hand.len() < 5 {
                current_hand.push(card);
            } else {
                self.graveyard.push(card);
            }
        }

        // update the hand with the new card
        self.hand = current_hand.to_vec();
    }

    pub fn is_dead(&self) -> bool {
        self.health < 1
    }

    pub fn update_turn_mana(&mut self) {
        self.mana.increment_and_refresh_mana();
    }

    pub fn take_damage(&mut self, damage: i8) {
        self.health -= damage;
    }
}