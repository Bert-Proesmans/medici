//! EndTurn trigger related methods.

use failure::Error;

use game_system::prelude::prototype::Game as GameProto;
use game_system::prelude::*;
// use game_system::runtime::exec_triggers;

/// Defines a trigger which will be run when the turn of the current player ends.
pub fn turn_end_trigger<CTS>(
    mut x: Machine<Trigger<Peri, EndTurn>, CTS>,
) -> Result<Machine<Trigger<Peri, EndTurn>, CTS>, Error>
where
    CTS: CTStack + 'static,
{
    println!("[TURN_END_TRIGGER] PERI - END TURN");
    //
    let game_entity = x.entities.get_mut(GAME_E_ID)?;
    let mut game_proto = game_entity.as_proto_mut::<GameProto>()?;
    game_proto.set_next_player()?;
    //
    Ok(x)
}
