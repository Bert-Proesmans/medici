use std::collections::HashMap;
use std::vec::Vec;

use value_from_type_macros::value_from_type;

use medici_macros::{build_automaton, ActionState, GlobalState, WaitState};
use medici_traits::prelude::*;
use medici_traits::entities::{EntityPrototype, E_ID_KEY};
use medici_traits::automata::pushdown_automaton::{PullupFrom, PushdownFrom};

use containers::entities::EntityService;
use containers::listeners::ListenerService;
use containers::tapes::TapeService;

// TODO; Make the fields on these structs private?
build_automaton!{
    // Game object layout

    #[derive(Debug)]
    struct Game<X: Global> {
        // State MUST BE THE FIRST PARAMETER, defining X.
        // X represents on of the global states.
        pub state: X,
        pub listeners: ListenerService,
        pub entities: EntityService,
        pub storage: TapeService,
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
        pub card: &'static Card,
    }

    #[derive(Debug)]
    struct Card {
        pub uid: CardId,
        pub name: &'static str,

        pub data: HashMap<GameTags, u32>,
    }

    states {
        // Each group represents a submodule (the names are converted to snake_case as well)
        Global {
            #[derive(Debug, GlobalState)]
            struct Wait<W: Waitable>(W);
            // Actionable inherits from Triggerable
            #[derive(Debug, GlobalState)]
            struct Action<U: Actionable>(U);
            #[derive(Debug, GlobalState)]
            struct Finished();

            #[derive(Debug, GlobalState)]
            struct Effect<U: Actionable>(U);
            #[derive(Debug, GlobalState)]
            struct Trigger<T: Timing, U: Triggerable>(T, U);
            #[derive(Debug, GlobalState)]
            struct Death<T: Timing, U: Triggerable>(T, U);

            // Custom states can be defined here.
            #[derive(Debug, GlobalState)]
            struct RecurseEffect<T: Timing, U: Triggerable>(T, U);
        }

        Waitable {
            #[derive(Debug, WaitState)]
            struct Start();

            #[derive(Debug, WaitState)]
            struct Input();
        }

        // You could also re-export defaults from medici_traits!
        // Pre / Peri / Post for timing_states
        Timing {
            use medici_traits::timing::default::{Pre, Peri, Post};
            use medici_traits::timing::default::EnumerationTiming;
        }

        Triggerable {
            #![value_from_type(EnumerationTrigger)]

            // EndTurn is Actionable, but we HAVE TO merge actionables and triggerables
            // together into one module to be able to build EnumerationTrigger from it.
            #[derive(Debug, ActionState)]
            struct EndTurn();
        }
    }

    /* Possible state machine transitions. */

    // Into's are non-reversible transitions.
    // Pushdowns are reversible transitions, eg A <-> B will result
    // in a pushdown from A into B, and a pullup from B into A.
    transitions {
        // Game<X> is implicit and can be ommited, since it would make
        // the syntax more difficult to read.
        into_transitions {
            Wait<Start> -> Wait<Input>,

            // Actions
            Wait<Input> -> Action<EndTurn>,
            Action<EndTurn> -> Wait<Input>,

            // End turn effect machine
            Effect<EndTurn> -> Trigger<Pre, EndTurn>,
            Trigger<Pre, EndTurn> -> Death<Pre, EndTurn>,
            Trigger<Peri, EndTurn> -> Death<Peri, EndTurn>,
            Trigger<Post, EndTurn> -> Death<Post, EndTurn>,
            Death<Pre, EndTurn> -> Trigger<Peri, EndTurn>,
            Death<Peri, EndTurn> -> Trigger<Post, EndTurn>,
            Death<Post, EndTurn> -> Effect<EndTurn>,
        }

        pushdown_transitions {
            Action<EndTurn> <-> Effect<EndTurn>,
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

/* DBG IMPLS */
// TODO; Move these definitions into the proc macro!

////////////////////////////
use self::states::global::{Death, RecurseEffect, Trigger};

impl<T, U> PushdownFrom<Game<Trigger<T, U>>> for Game<RecurseEffect<T, U>>
where
    T: Timing,
    U: Triggerable,
{
    fn pushdown_from(x: Game<Trigger<T, U>>) -> Self {
        Game {
            state: PhantomData,
            listeners: x.listeners,
            entities: x.entities,
            storage: x.storage,
        }
    }
}

impl<T, U> PullupFrom<Game<RecurseEffect<T, U>>> for Game<Trigger<T, U>>
where
    T: Timing,
    U: Triggerable,
{
    fn pullup_from(x: Game<RecurseEffect<T, U>>) -> Self {
        Game {
            state: PhantomData,
            listeners: x.listeners,
            entities: x.entities,
            storage: x.storage,
        }
    }
}

impl<T, U> PushdownFrom<Game<Death<T, U>>> for Game<RecurseEffect<T, U>>
where
    T: Timing,
    U: Triggerable,
{
    fn pushdown_from(x: Game<Death<T, U>>) -> Self {
        Game {
            state: PhantomData,
            listeners: x.listeners,
            entities: x.entities,
            storage: x.storage,
        }
    }
}

impl<T, U> PullupFrom<Game<RecurseEffect<T, U>>> for Game<Death<T, U>>
where
    T: Timing,
    U: Triggerable,
{
    fn pullup_from(x: Game<RecurseEffect<T, U>>) -> Self {
        Game {
            state: PhantomData,
            listeners: x.listeners,
            entities: x.entities,
            storage: x.storage,
        }
    }
}

////////////////////////////

impl<'a> EntityPrototype for prototypes::Game<'a> {}
impl<'a> From<&'a Entity> for prototypes::Game<'a> {
    fn from(e: &'a Entity) -> Self {
        prototypes::Game(&e)
    }
}

////////////////////////////

// impl<W: Waitable> Global for self::states::global::Wait<W> {}
// impl<T: Timing, U: Actionable> Global for self::states::global::Action<T, U> {}
// impl Global for self::states::global::Finished {}
// impl<T: Timing, U: Triggerable> Global for self::states::global::Effect<T, U> {}
// impl<T: Timing, U: Triggerable> Global for self::states::global::Trigger<T, U> {}
// impl<T: Timing, U: Actionable> Global for self::states::global::Death<T, U> {}

// impl Waitable for self::states::waitable::Input {}

// impl Actionable for self::states::actionable::EndTurn {}
// impl Triggerable for self::states::actionable::EndTurn {}

// This enumeration holds all tags which can describe properties of
// entities.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum GameTags {
    EntityId = E_ID_KEY,
}

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;
    // use medici_traits::prelude::*;
    use medici_traits::automata::{PullupInto, PushdownInto};

    use super::*;
    use super::states::global::*;
    use super::states::waitable::*;
    // use super::states::timing::*;
    use super::states::triggerable::*;
    use super::prototypes::Game as GamePrototype;

    #[test]
    fn game_transitions() {
        let game: Game<Wait<Input>> = Game {
            state: PhantomData,
            listeners: ListenerService::new(),
            entities: EntityService::new(),
            storage: TapeService::new(),
        };

        let game: Game<Action<EndTurn>> = game.into();
        let game: Game<Effect<EndTurn>> = game.pushdown();
        let _game: Game<Action<EndTurn>> = game.pullup();
    }

    #[test]
    fn prototypes() {}
}
