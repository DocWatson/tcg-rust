pub mod card;
pub mod deck;
pub mod mana;
pub mod menu;
pub mod player;

use self::player::Player;

#[derive(Debug)]
pub struct Game {
    player1: Player,
    player2: Player,
}

impl Game {
    pub fn new(player1_name: String) -> Game {
        let player1 = Player::new(player1_name);
        let player2 = Player::new("Jimmy".to_string());

        Game {
            player1: player1,
            player2: player2
        }
    }

    pub fn game_turn(&mut self) {
        if self.game_is_over() {
            println!("SOMEONE DIED!!");
        } else {
            &self.player1.update_turn_mana();
            &self.player2.update_turn_mana();

            &self.player1.draw();
            &self.player2.draw();
            
            self.game_turn();
        }
    }

    fn game_is_over(&mut self) -> bool {
        if self.player1.is_dead() {
            if self.player2.is_dead() {
                println!("Draw!");
                return true
            }
            println!("Player 1 loses");
            return true
        } else if self.player2.is_dead() {
            println!("Player 2 loses");
            return true
        }

        return false;
    }
}
