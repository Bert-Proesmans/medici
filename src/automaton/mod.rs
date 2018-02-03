mod config;
pub mod implementations;
pub mod card_sets;
pub mod runtime;

mod setup {
    use std::marker::PhantomData;
    use containers::listeners::ListenerService;
    use containers::entities::EntityService;
    use containers::tapes::TapeService;

    use super::prelude::*;

    impl Game<Wait<Input>> {
        pub fn new() -> Self {
            Game {
                state: PhantomData,
                entities: EntityService::new(),
                storage: TapeService::new(),
                listeners: ListenerService::new(),
            }
        }
    }
}

/* Broker overlay between configured automaton and medici implementations. */

pub mod prelude {
    // Make sure to NOT export
    //      super::config::states::*
    // because all states are leaked into that module because of access limitations
    // when building transition methods!

    pub use super::config::{Entity, Game, GameTags, Card};
    pub use super::config::states::global::{Action, Death, Effect, Finished, Trigger, Wait};
    pub use super::config::states::waitable::Input;
    pub use super::config::states::timing::{Peri, Post, Pre};
    pub use super::config::states::triggerable::EndTurn;
    // pub use super::config::states::triggerable::*;
    // Don't confuse this type with the Game structure!
    pub use super::config::prototypes::Game as GameProto;
}

pub mod states {
    pub use super::config::states::global;
    pub use super::config::states::waitable;
    pub use super::config::states::timing;
    pub use super::config::states::triggerable;
}

pub mod prototypes {
    pub use super::config::prototypes::*;
}

/* ************************** */
