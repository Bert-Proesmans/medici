//! Implementation of the Game prototype.

use failure::Error;

use tag::EntityTags;

use super::Game as GameProto;

impl<'a> GameProto<'a> {
    pub fn set_next_player(&mut self) -> Result<(), Error> {
        let ref mut g = self.0.unwrap_mut()?;
        let max_players = g.get_value(&EntityTags::MaxPlayers)?;
        let current_ord = g.get_value(&EntityTags::CurrentPlayerOrd)?;
        // Calculate the next playerID.
        // 1-indexed!
        let mut next_ord = current_ord + 1;
        if next_ord > max_players {
            next_ord = 1;
        }
        g.set_value(EntityTags::CurrentPlayerOrd, next_ord);
        Ok(())
    }
}
