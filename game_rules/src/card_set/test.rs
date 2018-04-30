//! This module contains all game cards which are part of the test-set.

use game_system::card_impl;
use game_system::prelude::*;

use super::CardSet;

lazy_static! {
    static ref TST_01: Card = {
        card_impl!{
            ID = CardId::new(CardSet::Test as u16, 1);
            NAME = "Wizard";
            properties {
                EntityTags::Attack = 5;
                EntityTags::Health = 3;
            }

            triggers {
                print_on_play [
                    TIMING = Peri;
                    TRIGGER =  PlayCard;
                ] => |machine| {
                    println!("TEST");
                    Ok(machine)
                }

                print_on_start [
                    TIMING = Peri;
                    TRIGGER = Start;
                ] => |machine| {
                    println!("Printing on Start");
                    Ok(machine)
                }
            }
        }
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
