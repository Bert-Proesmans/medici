use automata::pushdown_automaton::{Pullup, Pushdown};
use containers::games::Game;
use hs_automaton::states::*;
use hs_automaton::states::global_states::timing;

impl Pushdown<Game<Effect<timing::Pre, EndTurn>>> for Game<Action<timing::Pre, EndTurn>> {
    fn pushdown(self) -> Game<Effect<timing::Pre, EndTurn>> {
        Game {
            state: Effect(timing::Pre(), EndTurn()),
            listeners: self.listeners,
            entities: self.entities,
            storage: self.storage,
        }
    }
}

impl Pullup<Game<Effect<timing::Pre, EndTurn>>> for Game<Action<timing::Pre, EndTurn>> {
    fn pullup(x: Game<Effect<timing::Pre, EndTurn>>) -> Self {
        Game {
            state: Action {
                timing: timing::Pre(),
                activity: EndTurn(),
            },
            listeners: x.listeners,
            entities: x.entities,
            storage: x.storage,
        }
    }
}

impl Pushdown<Game<Trigger<timing::Pre, EndTurn>>> for Game<Effect<timing::Pre, EndTurn>> {
    fn pushdown(self) -> Game<Trigger<timing::Pre, EndTurn>> {
        Game {
            state: Trigger(timing::Pre(), EndTurn()),
            listeners: self.listeners,
            entities: self.entities,
            storage: self.storage,
        }
    }
}

impl Pullup<Game<Trigger<timing::Pre, EndTurn>>> for Game<Effect<timing::Pre, EndTurn>> {
    fn pullup(x: Game<Trigger<timing::Pre, EndTurn>>) -> Self {
        Game {
            state: Effect(timing::Pre(), EndTurn()),
            listeners: x.listeners,
            entities: x.entities,
            storage: x.storage,
        }
    }
}

impl Pushdown<Game<Trigger<timing::Peri, EndTurn>>> for Game<Trigger<timing::Pre, EndTurn>> {
    fn pushdown(self) -> Game<Trigger<timing::Peri, EndTurn>> {
        Game {
            state: Trigger(timing::Peri(), EndTurn()),
            listeners: self.listeners,
            entities: self.entities,
            storage: self.storage,
        }
    }
}

impl Pullup<Game<Trigger<timing::Peri, EndTurn>>> for Game<Trigger<timing::Pre, EndTurn>> {
    fn pullup(x: Game<Trigger<timing::Peri, EndTurn>>) -> Self {
        Game {
            state: Trigger(timing::Pre(), EndTurn()),
            listeners: x.listeners,
            entities: x.entities,
            storage: x.storage,
        }
    }
}

impl Pushdown<Game<Trigger<timing::Post, EndTurn>>> for Game<Trigger<timing::Peri, EndTurn>> {
    fn pushdown(self) -> Game<Trigger<timing::Post, EndTurn>> {
        Game {
            state: Trigger(timing::Post(), EndTurn()),
            listeners: self.listeners,
            entities: self.entities,
            storage: self.storage,
        }
    }
}

impl Pullup<Game<Trigger<timing::Post, EndTurn>>> for Game<Trigger<timing::Peri, EndTurn>> {
    fn pullup(x: Game<Trigger<timing::Post, EndTurn>>) -> Self {
        Game {
            state: Trigger(timing::Peri(), EndTurn()),
            listeners: x.listeners,
            entities: x.entities,
            storage: x.storage,
        }
    }
}
