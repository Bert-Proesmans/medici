//! Contains the core card set.
//! Core cards are necessary at minimum to properly start a game.

use std::fmt::Debug;

use game_system::card::{GAME_CARD_ID, PLAYER_CARD_ID};
use game_system::prelude::*;

// TODO; Build macro for card creation!
lazy_static! {
    static ref GAME_CARD: Card = {
        let mut c = Card::new_with_id(GAME_CARD_ID);
        c.name = "Game card";
        // TODO; Some properties might be needed?

        /* Triggers */
        pub fn print_on_start<CTS>(
            x: Machine<Trigger<Peri, Start>, CTS>,
        ) -> Result<Machine<Trigger<Peri, Start>, CTS>, MachineError>
        where
            CTS: CTStack + Debug + Clone + Send + 'static,
        {
            println!("[TRIGGER]\tGame Started!");
            Ok(x)
        }

        c.triggers.add_trigger(print_on_start::<AnyStack>);
        //
        c
    };

    static ref PLAYER_CARD: Card = {
        let mut c = Card::new_with_id(PLAYER_CARD_ID);
        c.name = "Player card";
        // TODO; Some properties might be needed?
        c
    };
}
