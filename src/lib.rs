#![feature(proc_macro)]
#![feature(attr_literals)] // Used for 'from_generic_derive' macro
#![feature(conservative_impl_trait)] // Used for 'fn() -> impl Iterator<Item=X>'
#![feature(try_from)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate maplit;

extern crate medici_macros;
extern crate medici_traits;

extern crate value_from_type_macros;
extern crate value_from_type_traits;

mod containers;
// Contains our custom state machine.
pub mod automaton;

#[cfg(test)]
mod tests {
    use std::default::Default;

    use medici_traits::entities::GAME_E_ID;

    use automaton::prelude::*;
    use automaton::implementations::effects::triggers::turn_end_trigger;
    use automaton::implementations::effects::actions::end_turn;

    use containers::cards::CardContainer;

    #[test]
    fn entry() {
        let config: SetupConfig = Default::default();
        let mut game = Game::new(config).expect("Error creating new game!");

        {
            let game_entity = game.entities.entity(GAME_E_ID).unwrap();
            assert_eq!(GAME_E_ID, game_entity.into());
        }

        // Add trigger
        game.listeners.add_trigger(turn_end_trigger).unwrap();

        // Start game
        let game: Game<Wait<Input>> = game.into();

        // Do stuff
        let first_turn = end_turn(game).expect("Game unexpectedly finished");
        let _second_turn = end_turn(first_turn).expect("Game unexpectedly finished");

        println!("OK - Finished");
    }

    #[test]
    fn listeners() {
        let config: SetupConfig = Default::default();
        let mut new_game = Game::new(config).expect("Error creating new game!");

        // Add trigger
        new_game.listeners.add_trigger(turn_end_trigger).unwrap();

        // Start game
        let new_game: Game<Wait<Input>> = new_game.into();

        // Do stuff
        let first_turn = end_turn(new_game).expect("Game unexpectedly finished");
        let _second_turn = end_turn(first_turn).expect("Game unexpectedly finished");
    }

    #[test]
    fn entities() {
        let game_card = CardContainer::game_card();
        let mut game_entity = Entity::new(GAME_E_ID, game_card);
        game_entity
            .add_proto::<GameProto>()
            .expect("Error in proto assignment!");
        game_entity
            .as_proto::<GameProto>()
            .expect("Error in proto retrieval!");

        game_entity
            .add_proto::<GameProtoMut>()
            .expect("Error in proto assignment!");
        game_entity
            .as_proto_mut::<GameProtoMut>()
            .expect("Error in proto retrieval!");
    }
}
