#![feature(nll, proc_macro)]
#![deny(missing_docs)]

//! Crate implementing a basic card game.

extern crate failure;
extern crate game_system;

mod action;
mod card_set;
mod trigger;

use std::default::Default;

use failure::{format_err, Error};
use game_system::prelude::*;

use action::{end_turn, start_game};
use trigger::{start_game_trigger, turn_end_trigger};

fn pre_end_turn_trigger<CTS>(
    x: Machine<Trigger<Pre, EndTurn>, CTS>,
) -> Result<Machine<Trigger<Pre, EndTurn>, CTS>, Error>
where
    CTS: CTStack,
{
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
        .add_trigger(start_game_trigger::<AnyStack>)
        .unwrap();
    wait_start_state
        .triggers
        .add_trigger(turn_end_trigger::<AnyStack>)
        .unwrap();
    wait_start_state
        .triggers
        .add_trigger(pre_end_turn_trigger::<AnyStack>)
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
