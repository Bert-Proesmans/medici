//! Contains the core card set.
//! Core cards are necessary at minimum to properly start a game.

use game_system::card::{GAME_CARD_ID, PLAYER_CARD_ID};
use game_system::prelude::*;

lazy_static! {
	static ref GAME_CARD: Card = {
		let mut c = Card::new_with_id(GAME_CARD_ID);
		c.name = "Game card";
		// TODO; Some properties might be needed?
		c
	};

	static ref PLAYER_CARD: Card = {
		let mut c = Card::new_with_id(PLAYER_CARD_ID);
		c.name = "Player card";
		// TODO; Some properties might be needed?
		c
	};
}

