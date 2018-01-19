#![feature(proc_macro)]
#![feature(attr_literals)] // Used for 'from_generic_derive' macro
#![feature(conservative_impl_trait)] // Used for 'fn() -> impl Iterator<Item=X>'
#![feature(try_from)]

extern crate medici_macros;
extern crate medici_traits;

extern crate value_from_type_macros;
extern crate value_from_type_traits;

mod containers;
mod hs_automaton;

#[cfg(test)]
mod tests {
    use containers::games::Game;
    use containers::listeners::ListenerService;
    use containers::entities::EntityService;
    use containers::tapes::TapeService;

    use hs_automaton::effects::triggers::turn_end_trigger;
    use hs_automaton::effects::actions::end_turn;
    use hs_automaton::states::*;

    #[test]
    fn entry() {
    let mut new_game = Game {
        state: Wait { activity: Input() },
        entities: EntityService {},
        storage: TapeService {},
        listeners: ListenerService {
            pre_actions: Vec::new(),
            peri_actions: Vec::new(),
            post_actions: Vec::new(),
            pure_triggers: Vec::new(),
        },
    };

        // Add trigger
        new_game
            .listeners
            .add_pure_trigger(turn_end_trigger)
            .unwrap();

        // Do stuff
        let first_turn = end_turn(new_game).expect("Game finished");
        let second_turn = end_turn(first_turn).expect("Game finished");

        // let item = Game { state: Pre(Action { activity: EndTurn() }) };

        // // let pushed: Game<Pre<Trigger<Pre<EndTurn>>>> = item.pushdown();
        // let pushed = item.pushdown();
        // // let item = pushed.pullup();

        println!("OK - Finished");
    }
}
