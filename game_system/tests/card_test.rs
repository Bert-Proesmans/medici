#![feature(use_extern_macros)]

#[macro_use]
extern crate game_system;

use std::fmt::Debug;

use game_system::prelude::*;

#[test]
/// Creates a new card with added triggers.
fn card_creation() {
    let mut c = Card::new_with_id(CardId::new(5, 2));
    c.name = "Game card";
    c.set_value(EntityTags::Attack, 5);
    c.set_value(EntityTags::Health, 10);

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

    pub fn after_played_card<CTS>(
        x: Machine<Trigger<Post, PlayCard>, CTS>,
    ) -> Result<Machine<Trigger<Post, PlayCard>, CTS>, MachineError>
    where
        CTS: CTStack + Debug + Clone + Send + 'static,
    {
        let card_name = "";
        println!("[TRIGGER]\tAfter the card {} has been played!", card_name);
        Ok(x)
    }
    c.triggers.add_trigger(after_played_card::<AnyStack>);
}

#[test]
fn macro_impl() {
    let _c = card_impl!{
        ID = CardId::new(1, 1);
        NAME = "Wizard";
    };

    let _c = card_impl!{
        ID = CardId::new(1, 1);
        NAME = "Wizard";
        properties {
            EntityTags::Attack = 5;
            EntityTags::Health = 3;
        }
    };

    let _c = card_impl!{
        ID = CardId::new(1, 1);
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
    };
}
