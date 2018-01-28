use std::collections::HashMap;
use value_from_type_macros::value_from_type;
use medici_macros::{build_automaton, WaitState, GlobalState, ActionState};
use medici_traits::prelude::*;
use medici_traits::entities::{E_ID_KEY, EntityPrototype};
use containers::entities::EntityService;
// use containers::listeners::ListenerService;
use containers::tapes::TapeService;

build_automaton!{
    // Game object layout

    #[derive(Debug)]
    struct Game<X: Global> {
        // State MUST BE THE FIRST PARAMETER, defining X.
        // X represents on of the global states.
        state: X,
        // listeners: ListenerService,
        entities: EntityService,
        storage: TapeService,
    }

    #[derive(Debug)]
    struct Entity {
        /* Entity Data */
        pub id: EntityId,
        // GameTags is an enum, defined below
        pub state: HashMap<GameTags, u32>,
        
        /* other stuff */
        // This points towards the generated enum from the prototypes module, see below.
        pub prototypes: Vec<prototypes::EnumerationPrototype>,
        pub card: u32 // TODO
    }

    states {
        // Each group represents a submodule (the names are converted to snake_case as well)
        Global {
            #[derive(Debug, GlobalState)]
            struct Wait<W: Waitable>(W);
            // Actionable inherits from Triggerable
            #[derive(Debug, GlobalState)]
            struct Action<T: Timing, U: Actionable>(T, U);
            #[derive(Debug, GlobalState)]
            struct Finished();

            #[derive(Debug, GlobalState)]
            struct Effect<T: Timing, U: Triggerable>(T, U);
            #[derive(Debug, GlobalState)]
            struct Trigger<T: Timing, U: Triggerable>(T, U);
            #[derive(Debug, GlobalState)]
            struct Death<T: Timing, U: Actionable>(T, U);

            // Custom states can be defined here.
        }

        Waitable {
            #[derive(Debug, WaitState)]
            struct Input();
        }

        // You could also re-export defaults from medici_traits!
        // Pre / Peri / Post for timing_states
        Timing {
            use medici_traits::timing::default::{Pre, Peri, Post};
            use medici_traits::timing::default::EnumerationTiming;
        }

        Actionable {
            #[derive(Debug, ActionState)]
            struct EndTurn();
        }

        Triggerable {}
    }

    /* Possible state machine transitions. */

    // Into's are non-reversible transitions.
    // Pushdowns are reversible transitions, eg A <-> B will result
    // in a pushdown from A into B, and a pullup from B into A.
    transitions {
        // Game<X> is implicit and can be ommited, since it would make
        // the syntax more difficult to read.
        into_transitions {
            Wait<Input> -> Action<Pre, EndTurn>,
            Death<Post, EndTurn> -> Wait<Input>,

            Action<Pre, EndTurn> -> Death<Pre, EndTurn>,
            Death<Pre, EndTurn> -> Action<Peri, EndTurn>,
            Action<Peri, EndTurn> -> Death<Peri, EndTurn>,
            Death<Peri, EndTurn> -> Action<Post, EndTurn>,
            Action<Post, EndTurn> -> Death<Post, EndTurn>,
        }

        pushdown_transitions {
            Action<Pre, EndTurn> <-> Effect<Pre, EndTurn>,
            Effect<Pre, EndTurn> <-> Trigger<Pre, EndTurn>,
            Trigger<Pre, EndTurn> <-> Trigger<Peri, EndTurn>,
            Trigger<Peri, EndTurn> <-> Trigger<Post, EndTurn>,
        }
    }

    // Behaviours for our entities. Each entry will become a tuple struct 
    // and it's body will be the implementation of that struct.
    prototypes {
        #![value_from_type(EnumerationPrototype)]
        
        // Example of Game prototype:
        //      Game<'a>(&'a Entity);
        Game {
            // This is the implementation of the GamePrototype.
            // The relevant Entity can be accessed by using self.0.
        },
    }
}

// This enumeration holds all tags which can describe properties of 
// entities.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum GameTags {
    EntityId = E_ID_KEY,
}

/* DBG IMPLS */
impl<'a> EntityPrototype for prototypes::Game<'a> {}
// impl<W: Waitable> Global for self::states::global::Wait<W> {}
// impl<T: Timing, U: Actionable> Global for self::states::global::Action<T, U> {}
// impl Global for self::states::global::Finished {}
// impl<T: Timing, U: Triggerable> Global for self::states::global::Effect<T, U> {}
// impl<T: Timing, U: Triggerable> Global for self::states::global::Trigger<T, U> {}
// impl<T: Timing, U: Actionable> Global for self::states::global::Death<T, U> {}

// impl Waitable for self::states::waitable::Input {}

// impl Actionable for self::states::actionable::EndTurn {}
// impl Triggerable for self::states::actionable::EndTurn {}

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;
    use medici_traits::prelude::*;
    use medici_traits::automata::{PullupInto, PushdownInto};

    use super::*;
    use super::states::global::*;
    use super::states::waitable::*;
    use super::states::timing::*;
    use super::states::actionable::*;
    use super::prototypes::Game as GamePrototype;

    #[test]
    fn game_transitions() {
        let game: Game<Wait<Input>> = Game {
            state: PhantomData,
            entities: EntityService::new(),
            storage: TapeService::new(),
        };

        let game: Game<Action<Pre, EndTurn>> = game.into();
        let game: Game<Effect<Pre, EndTurn>> = game.pushdown();
        let game: Game<Action<Pre, EndTurn>> = game.pullup();
    }

    #[test]
    fn prototypes() {
        
    }
}
