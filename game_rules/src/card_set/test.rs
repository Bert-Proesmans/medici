//! This module contains all game cards which are part of the test-set.

use game_system::card_impl;
use game_system::prelude::*;

use super::CardSet;

lazy_static! {
    static ref TST_01: Card = {
        card_impl!{
            ID = CardId::from_set(CardSet::Test, 1);
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
                    println!("Wizard played");
                    Ok(machine)
                }

                print_on_start [
                    TIMING = Peri;
                    TRIGGER = Start;
                ] => |machine| {
                    println!("Wizard says: Game Start");
                    Ok(machine)
                }
            }
        }
    };
    static ref TST_02: Card = {
        card_impl!{
            ID = CardId::from_set(CardSet::Test, 2);
            NAME = "Wizard 2";
            properties {
                EntityTags::Attack = 3;
                EntityTags::Health = 5;
            }

            triggers {
                print_on_play [
                    TIMING = Peri;
                    TRIGGER =  PlayCard;
                ] => |machine| {
                    println!("Wizard 2 played");
                    Ok(machine)
                }

                print_on_start [
                    TIMING = Peri;
                    TRIGGER = Start;
                ] => |machine| {
                    println!("Wizard 2 says: Game Start");
                    Ok(machine)
                }
            }
        }
    };
}

lazy_static! {
    // Collects all defined cards into one handy iterator.
    // The amount of items within the slice type must be updated
    static ref ALL_CARDS: [&'static Card; 2] = {
        [
            &TST_01,
            &TST_02,
            //
        ]
    };
}
