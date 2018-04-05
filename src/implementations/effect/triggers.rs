use medici_traits::entities::GAME_E_ID;

use automaton::prelude::*;

pub fn turn_end_trigger(
    mut x: Game<Trigger<Peri, EndTurn>>,
) -> Result<Game<Trigger<Peri, EndTurn>>, Game<Finished>> {
    println!("[TURN_END_TRIGGER] PERI - END TURN");
    {
    	let game_entity = x.entities.entity_mut(GAME_E_ID)
    							.ok_or_else(|| Default::default())?;

    	let mut game_proto = game_entity
				            .as_proto_mut::<GameProtoMut>()
				            .map_err(|_| Default::default())?; // "Error in proto retrieval!"

		game_proto.set_next_player().map_err(|_| Default::default())?;
    }
    Ok(x)
}
