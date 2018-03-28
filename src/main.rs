extern crate colored;
use self::colored::*;

pub mod game;

fn main() {
    print!("Enter your name:  ");
    let s = game::menu::get_input();

    let mut thegame = game::Game::new(s);
    println!("{:?}", thegame);

    thegame.game_turn();
    println!("{}", "======================".dimmed());
    println!("{:?}", thegame);
}
