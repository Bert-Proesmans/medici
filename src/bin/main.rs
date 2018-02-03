extern crate medici;

use medici::automaton::prelude::*;
use medici::automaton::implementations::effects::triggers::turn_end_trigger;
use medici::automaton::implementations::effects::actions::end_turn;

fn main() {
    // Enable backtraces without messing with env.
    // std::env::set_var("RUST_BACKTRACE", "1");

    println!("Starting - Running MAIN");
    let mut game = Game::new();

    // Add trigger
    game.listeners.add_trigger(turn_end_trigger).unwrap();
    println!("Listener added");

    // Do stuff
    println!("Ending turn P1");
    let first_turn = end_turn(game).expect("Game unexpectedly finished");
    println!("Ending turn P2");
    let _second_turn = end_turn(first_turn).expect("Game unexpectedly finished");

    println!("FINISHED");
}
