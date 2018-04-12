extern crate medici;
extern crate medici_core;

use std::default::Default;

use medici_core::stm::*;

use medici::state_machine::prelude::*;
use medici::state_machine::state::prelude::*;
use medici::state_machine::transaction::*;

fn main() {
    // DBG; This will enable Failure to print out full backtraces.
    // env::set_var("RUST_BACKTRACE", "1");

    let game_config = Default::default();
    let wait_start_state = Machine::new(&game_config).expect("Game setup error");

    // DBG; The following syntax can/will be made simpler by implementing the TransitionInto-
    // counterpart of TransitionFrom.
    let input_state: Machine<Wait<Input>> = wait_start_state.transition(Epsilon);

    let action_state: Machine<Action<Print>> = input_state.pushdown(PrintTransaction("Hello"));

    println!("Printing transaction: {:?}", action_state.transaction);

    let deep_action_state: Machine<Action<Load>> = action_state.pushdown(Epsilon);

    let action_state: Machine<Action<Print>> =
        deep_action_state.pullup().expect("Transition Error");

    println!("Validate transaction: {:?}", action_state.transaction);

    let input_state: Machine<Wait<Input>> = action_state.pullup().expect("Transition Error");

    // TODO: Fix
    /*
    let finished_state: Machine<Finished> = input_state.transition(Epsilon);

    println!("{:?}", finished_state);
    */}

fn _old_main() {
    // Enable backtraces without messing with env.
    // std::env::set_var("RUST_BACKTRACE", "1");

    // println!("Starting - Running MAIN");
    // let config: SetupConfig = Default::default();
    // let mut game = Game::new(config).expect("Error creating new game!");

    // // Add trigger
    // game.listeners.add_trigger(turn_end_trigger).unwrap();
    // println!("Listener added");

    // // Start the game
    // let game: Game<Wait<Input>> = game.transition(Epsilon());

    // // Do stuff
    // println!("Ending turn P1");
    // let first_turn = end_turn(game).expect("Game unexpectedly finished");
    // println!("Ending turn P2");
    // let _second_turn = end_turn(first_turn).expect("Game unexpectedly finished");

    // println!("FINISHED");
}
