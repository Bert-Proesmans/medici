//! This module contains all game cards which are part of the test-set.

use super::CardSet;
use game_system::prelude::*;

lazy_static! {
    static ref TST_01: Card = {
        let mut c = Card::new_with_id(CardId::new(CardSet::Test as u16, 1));
        c.name = "Wizard";
        c.set_value(EntityTags::Attack, 5);
        c.set_value(EntityTags::Health, 3);
        c
    };
}

lazy_static! {
    // Collects all defined cards into one handy iterator.
    // The amount of items within the slice type must be updated
    static ref ALL_CARDS: [&'static Card; 1] = {
        [
            &TST_01,
            //
        ]
    };
}
