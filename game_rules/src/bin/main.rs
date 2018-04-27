extern crate game_rules;

use game_rules::action::*;
use game_rules::game_system::prelude::*;
use game_rules::trigger::*;

fn main() {
    // DBG; This will enable Failure to print out full backtraces.
    // env::set_var("RUST_BACKTRACE", "1");

    let game_config = Default::default();
    let mut wait_start_state = Machine::new(&game_config).expect("Game setup error");

    // Add triggers
    wait_start_state
        .triggers
        .add_trigger(start_game_trigger::<AnyStack>);
    wait_start_state
        .triggers
        .add_trigger(turn_end_trigger::<AnyStack>);
    wait_start_state
        .triggers
        .add_trigger(pre_end_turn_trigger::<AnyStack>);

    // Start the game, which will start the turn of the first player.
    let first_turn = start_game(wait_start_state).expect("Game didn't start!");
    let second_turn = end_turn(first_turn).expect("Game unexpectedly finished!");
    let _third_turn = end_turn(second_turn).expect("Game unexpectedly finished!");

    // TODO: Fix
    /*
    let finished_state: Machine<Finished> = input_state.transition(Epsilon);

    println!("{:?}", finished_state);
    */
    println!("Finished");
}
