#![feature(nll)]

#[macro_use]
extern crate failure;
extern crate medici;
extern crate medici_core;

use std::default::Default;

use failure::Error;

use medici_core::prefab::entity::GAME_E_ID;
// use medici_core::stm::*;

use medici::implementation::effect::action::{end_turn, start_game};
use medici::implementation::effect::trigger::{start_game_trigger, turn_end_trigger};
use medici::implementation::entity::EntityTags;
use medici::state_machine::prelude::*;
use medici::state_machine::state::prelude::*;
// use medici::state_machine::transaction::*;

fn pre_end_turn_trigger(
    x: Machine<Trigger<Pre, EndTurn>>,
) -> Result<Machine<Trigger<Pre, EndTurn>>, Error> {
    let game_entity = x.entities.get(GAME_E_ID)?;
    let player_idx = game_entity
        .get_value(&EntityTags::CurrentPlayerOrd)
        .ok_or_else(|| format_err!("Missing CurrentPlayerOrd!"))?;
    println!("[PRE_ENDTURN_TRIGGER] for player {:}", player_idx);
    //
    Ok(x)
}

fn main() {
    // DBG; This will enable Failure to print out full backtraces.
    // env::set_var("RUST_BACKTRACE", "1");

    let game_config = Default::default();
    let mut wait_start_state = Machine::new(&game_config).expect("Game setup error");

    // Add triggers
    wait_start_state
        .triggers
        .add_trigger(start_game_trigger)
        .unwrap();
    wait_start_state
        .triggers
        .add_trigger(turn_end_trigger)
        .unwrap();
    wait_start_state
        .triggers
        .add_trigger(pre_end_turn_trigger)
        .unwrap();

    // Start the game, which will start the turn of the first player.
    let first_turn = start_game(wait_start_state).expect("Game didn't start!");
    let second_turn = end_turn(first_turn).expect("Game unexpectedly finished!");
    let _third_turn = end_turn(second_turn).expect("Game unexpectedly finished!");

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
