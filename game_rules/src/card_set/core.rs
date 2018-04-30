//! Contains the core card set.
//! Core cards are necessary at minimum to properly start a game.

use game_system::card::{GAME_CARD_ID, PLAYER_CARD_ID};
use game_system::card_impl;
use game_system::prelude::*;

lazy_static! {
    // Here additional game- and player cards can be defined.

    /// Example for when players want to play in hardcore mode.
    static ref HARDCORE_PLAYER_CARD: Card = {
        card_impl!{
            ID = PLAYER_CARD_ID;
            NAME = "Player card";

            properties {
                // Start with half the HP
                EntityTags::Health = 15;
                EntityTags::StartHandSize = 5;
            }
        }
    };
}
