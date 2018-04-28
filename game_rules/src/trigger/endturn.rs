//! EndTurn trigger related methods.
use std::fmt::Debug;

use game_system::prelude::prototype::Game as GameProto;
use game_system::prelude::*;
// use game_system::runtime::exec_triggers;

/// DBG
pub fn pre_end_turn_trigger<CTS>(
    x: Machine<Trigger<Pre, EndTurn>, CTS>,
) -> Result<Machine<Trigger<Pre, EndTurn>, CTS>, MachineError>
where
    CTS: CTStack + Debug + Clone + Send + 'static,
{
    let game_entity = ctxt!(x.entities.get(GAME_E_ID); x);
    let player_idx = ctxt!(game_entity.get_value(&EntityTags::CurrentPlayerOrd); x);
    println!("[PRE_ENDTURN_TRIGGER] for player {:}", player_idx);
    //
    Ok(x)
}

/// Defines a trigger which will be run when the turn of the current player ends.
pub fn turn_end_trigger<CTS>(
    mut x: Machine<Trigger<Peri, EndTurn>, CTS>,
) -> Result<Machine<Trigger<Peri, EndTurn>, CTS>, MachineError>
where
    CTS: CTStack + Debug + Clone + Send + 'static,
{
    println!("[TURN_END_TRIGGER] PERI - END TURN");
    //
    let game_entity = ctxt!(x.entities.get_mut(GAME_E_ID); x);
    let mut game_proto =
        ctxt!(game_entity.as_proto_mut::<GameProto>(); ErrorKind::ConstraintError, x);
    hydrate!(game_proto.set_next_player(); x);
    //
    Ok(x)
}
