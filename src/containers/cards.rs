use std::collections::HashMap;

use medici_traits::prelude::*;

use automaton::prelude::Card;

lazy_static!{
    static ref GAME_CARD: Card = Card {
        uid: 0,
        name: "GAME",
        data: hashmap!{},
    };

    static ref PLAYER_CARD: Card = Card {
        uid: 0,
        name: "PLAYER",
        data: hashmap!{},
    };

    pub static ref ALL_CARDS: CardContainer = {
        CardContainer::new()
    };
}

pub struct CardContainer {
    cards: Option<HashMap<CardId, Card>>,
}

impl CardContainer {
    fn new() -> Self {
        CardContainer { cards: None }
    }

    pub fn game_card() -> &'static Card {
        &*GAME_CARD
    }

    pub fn player_card() -> &'static Card {
        &*PLAYER_CARD
    }
}
