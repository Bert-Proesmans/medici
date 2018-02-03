use std::marker::PhantomData;

use containers::listeners::ListenerService;
use containers::entities::EntityService;
use containers::tapes::TapeService;
use containers::cards::CardContainer;

use automaton::prelude::*;

impl Game<Wait<Start>> {
    pub fn new(c: SetupConfig) -> Result<Self, ()> {
        let game = Game {
            state: PhantomData,
            entities: EntityService::new(&c)?,
            storage: TapeService::new(&c)?,
            listeners: ListenerService::new(&c)?,
        };

        Ok(game)
    }    
}
