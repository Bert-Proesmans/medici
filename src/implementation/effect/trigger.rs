//! Module containing the code for all triggers in the game.

use failure::Error;

// use medici_core::stm::*;
// use medici_core::prefab::entity::GAME_E_ID;

use state_machine::prelude::*;
use state_machine::state::prelude::*;
// use state_machine::transaction::*;

/// Defines a trigger which will be run when the turn of the current player ends.
pub fn turn_end_trigger(
    mut x: Machine<Trigger<Peri, EndTurn>>,
) -> Result<Machine<Trigger<Peri, EndTurn>>, Error> {
    println!("[TURN_END_TRIGGER] PERI - END TURN");
    /*
	let game_entity = x.entities.entity_mut(GAME_E_ID)
							.ok_or_else(|| Default::default())?;

	let mut game_proto = game_entity
			            .as_proto_mut::<GameProtoMut>()
			            .map_err(|_| Default::default())?; // "Error in proto retrieval!"

	game_proto.set_next_player().map_err(|_| Default::default())?;

    Ok(x)
    */
    unimplemented!()
}
