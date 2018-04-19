//! EndTurn trigger related methods.

use failure::{format_err, Error};

use game_system::prelude::prototype::Game as GameProto;
use game_system::prelude::*;
// use game_system::runtime::exec_triggers;

/// DBG
pub fn pre_end_turn_trigger<CTS>(
    x: Machine<Trigger<Pre, EndTurn>, CTS>,
) -> Result<Machine<Trigger<Pre, EndTurn>, CTS>, Error>
where
    CTS: CTStack,
{
    let game_entity = x.entities.get_entity(GAME_E_ID)?;
    let player_idx = game_entity
        .get_value(&EntityTags::CurrentPlayerOrd)
        .ok_or_else(|| format_err!("Missing CurrentPlayerOrd!"))?;
    println!("[PRE_ENDTURN_TRIGGER] for player {:}", player_idx);
    //
    Ok(x)
}

/// Defines a trigger which will be run when the turn of the current player ends.
pub fn turn_end_trigger<CTS>(
    mut x: Machine<Trigger<Peri, EndTurn>, CTS>,
) -> Result<Machine<Trigger<Peri, EndTurn>, CTS>, Error>
where
    CTS: CTStack + 'static,
{
    println!("[TURN_END_TRIGGER] PERI - END TURN");
    //
    let game_entity = x.entities.get_entity_mut(GAME_E_ID)?;
    let mut game_proto = game_entity.as_proto_mut::<GameProto>()?;
    game_proto.set_next_player()?;
    //
    Ok(x)
}
