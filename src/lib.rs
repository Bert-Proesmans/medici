#![feature(proc_macro)]
#![feature(attr_literals)] // Used for 'from_generic_derive' macro
#![feature(conservative_impl_trait)] // Used for 'fn() -> impl Iterator<Item=X>'
#![feature(try_from)]

#[macro_use]
extern crate maplit;

extern crate medici_macros;
extern crate medici_traits;

extern crate value_from_type_macros;
extern crate value_from_type_traits;

// mod containers;
mod hs_automaton;

#[cfg(test)]
mod tests {
    use containers::games::Game;
    use containers::listeners::ListenerService;
    use containers::entities::{Entity, EntityService, GAME_E_ID};
    use containers::tapes::TapeService;

    use hs_automaton::effects::triggers::turn_end_trigger;
    use hs_automaton::effects::actions::end_turn;
    use hs_automaton::states::*;

    use hs_automaton::entities::Game as GameEntity;

    impl Game<Wait<Input>> {
        pub fn new() -> Self {
            Game {
                state: Wait { activity: Input() },
                entities: EntityService::new(),
                storage: TapeService::new(),
                listeners: ListenerService::new(),
            }
        }
    }

    #[test]
    fn entry() {
        let mut game = Game::new();

        {
            let game_entity = game.entities.entity(GAME_E_ID).unwrap();
            assert_eq!(GAME_E_ID, game_entity.into());
        }

        // Add trigger
        game.listeners.add_pure_trigger(turn_end_trigger).unwrap();

        // Do stuff
        let first_turn = end_turn(game).expect("Game finished");
        let second_turn = end_turn(first_turn).expect("Game finished");

        // let item = Game { state: Pre(Action { activity: EndTurn() }) };

        // // let pushed: Game<Pre<Trigger<Pre<EndTurn>>>> = item.pushdown();
        // let pushed = item.pushdown();
        // // let item = pushed.pullup();

        println!("OK - Finished");
    }

    #[test]
    fn listeners() {
        let mut new_game = Game::new();

        // Add trigger
        new_game
            .listeners
            .add_pure_trigger(turn_end_trigger)
            .unwrap();

        // Do stuff
        let first_turn = end_turn(new_game).expect("Game finished");
        let second_turn = end_turn(first_turn).expect("Game finished");
    }

    #[test]
    fn entities() {
        let mut new_game = Game::new();

        let mut game_entity = Entity::new(GAME_E_ID, 0);
        game_entity.add_proto::<GameEntity>();
        let game_entity = game_entity.as_proto::<GameEntity>();
    }
}
