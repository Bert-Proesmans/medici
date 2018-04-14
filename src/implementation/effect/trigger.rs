//! Module containing the code for all triggers in the game.

use failure::Error;

use medici_core::prefab::entity::GAME_E_ID;

use implementation::entity::EntityTags;
use implementation::prototype::*;
use state_machine::prelude::*;
use state_machine::state::prelude::*;
// use state_machine::transaction::*;

/// Defines all activities which must happen when the game is started.
pub fn start_game_trigger(
    mut x: Machine<Trigger<Peri, Start>>,
) -> Result<Machine<Trigger<Peri, Start>>, Error> {
    println!("[START_GAME_TRIGGER] PERI - START");
    //
    // Set the current turn to be for player 1 (1), the first player.
    // Note that the value for CurrentPlayerOrd is 1-indexed!
    let mut game_entity = x.entities.get_mut(GAME_E_ID)?;
    game_entity.set_value(EntityTags::CurrentPlayerOrd, 1);
    Ok(x)
}

/// Defines a trigger which will be run when the turn of the current player ends.
pub fn turn_end_trigger(
    mut x: Machine<Trigger<Peri, EndTurn>>,
) -> Result<Machine<Trigger<Peri, EndTurn>>, Error> {
    println!("[TURN_END_TRIGGER] PERI - END TURN");
    //
    let game_entity = x.entities.get_mut(GAME_E_ID)?;
    let mut game_proto = game_entity.as_proto_mut::<Game>()?;
    game_proto.set_next_player()?;
    //
    Ok(x)
}
