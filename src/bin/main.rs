extern crate medici;

use std::default::Default;

use medici::automaton::prelude::*;
use medici::automaton::implementations::effects::triggers::turn_end_trigger;
use medici::automaton::implementations::effects::actions::end_turn;

fn main() {
    // Enable backtraces without messing with env.
    // std::env::set_var("RUST_BACKTRACE", "1");

    println!("Starting - Running MAIN");
    let config: SetupConfig = Default::default();
    let mut game = Game::new(config).expect("Error creating new game!");

    // Add trigger
    game.listeners.add_trigger(turn_end_trigger).unwrap();
    println!("Listener added");

    // Start the game
    let game: Game<Wait<Input>> = game.into();

    // Do stuff
    println!("Ending turn P1");
    let first_turn = end_turn(game).expect("Game unexpectedly finished");
    println!("Ending turn P2");
    let _second_turn = end_turn(first_turn).expect("Game unexpectedly finished");

    println!("FINISHED");
}
