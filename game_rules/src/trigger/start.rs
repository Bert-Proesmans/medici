//! Start trigger related methods.

use failure::Error;

use game_system::prelude::*;
// use game_system::runtime::exec_triggers;

/// Defines all activities which must happen when the game is started.
pub fn start_game_trigger<CTS>(
    mut x: Machine<Trigger<Peri, Start>, CTS>,
) -> Result<Machine<Trigger<Peri, Start>, CTS>, Error>
where
    CTS: CTStack + 'static,
{
    println!("[START_GAME_TRIGGER] PERI - START");
    //
    // Set the current turn to be for player 1 (1), the first player.
    // Note that the value for CurrentPlayerOrd is 1-indexed!
    let game_entity = x.entities.get_mut(GAME_E_ID)?;
    game_entity.set_value(EntityTags::CurrentPlayerOrd, 1);
    Ok(x)
}
