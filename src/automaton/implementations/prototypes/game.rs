use automaton::prelude::*;
use automaton::prototypes::{Game, GameMut};

impl<'a> GameMut<'a> {
    pub fn set_next_player(&mut self) -> Result<(), String> {
        let ref mut g = self.0;
        let max_players = g.get_value(GameTags::MaxPlayers).ok_or("No MaxPlayers")?;
        let current_ord = g.get_value(GameTags::CurrentPlayerOrd).ok_or("No CurrentPlayerOrd")?;
        let mut next_ord = current_ord + 1;
        if next_ord > max_players {
            // 1-indexed, reset to 1
            next_ord = 1;
        }
        g.set_value(GameTags::CurrentPlayerOrd, next_ord);
        Ok(())
    }
}
