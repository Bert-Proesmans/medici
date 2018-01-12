use automata::pushdown_automaton::{PullupFrom, PushdownFrom};
use containers::games::Game;
use hs_automaton::states::*;
use hs_automaton::states::global_states::timing;

impl PushdownFrom<Game<Action<timing::Pre, EndTurn>>> for Game<Effect<timing::Pre, EndTurn>> {
    fn pushdown_from(x: Game<Action<timing::Pre, EndTurn>>) -> Self {
        Game {
            state: Effect(timing::Pre(), EndTurn()),
            listeners: x.listeners,
            entities: x.entities,
            storage: x.storage,
        }
    }
}

impl PullupFrom<Game<Effect<timing::Pre, EndTurn>>> for Game<Action<timing::Pre, EndTurn>> {
    fn pullup_from(x: Game<Effect<timing::Pre, EndTurn>>) -> Self {
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

impl PushdownFrom<Game<Effect<timing::Pre, EndTurn>>> for Game<Trigger<timing::Pre, EndTurn>> {
    fn pushdown_from(x: Game<Effect<timing::Pre, EndTurn>>) -> Self {
        Game {
            state: Trigger(timing::Pre(), EndTurn()),
            listeners: x.listeners,
            entities: x.entities,
            storage: x.storage,
        }
    }
}

impl PullupFrom<Game<Trigger<timing::Pre, EndTurn>>> for Game<Effect<timing::Pre, EndTurn>> {
    fn pullup_from(x: Game<Trigger<timing::Pre, EndTurn>>) -> Self {
        Game {
            state: Effect(timing::Pre(), EndTurn()),
            listeners: x.listeners,
            entities: x.entities,
            storage: x.storage,
        }
    }
}

impl PushdownFrom<Game<Trigger<timing::Pre, EndTurn>>> for Game<Trigger<timing::Peri, EndTurn>> {
    fn pushdown_from(x: Game<Trigger<timing::Pre, EndTurn>>) -> Self {
        Game {
            state: Trigger(timing::Peri(), EndTurn()),
            listeners: x.listeners,
            entities: x.entities,
            storage: x.storage,
        }
    }
}

impl PullupFrom<Game<Trigger<timing::Peri, EndTurn>>> for Game<Trigger<timing::Pre, EndTurn>> {
    fn pullup_from(x: Game<Trigger<timing::Peri, EndTurn>>) -> Self {
        Game {
            state: Trigger(timing::Pre(), EndTurn()),
            listeners: x.listeners,
            entities: x.entities,
            storage: x.storage,
        }
    }
}

impl PushdownFrom<Game<Trigger<timing::Peri, EndTurn>>> for Game<Trigger<timing::Post, EndTurn>> {
    fn pushdown_from(x: Game<Trigger<timing::Peri, EndTurn>>) -> Self {
        Game {
            state: Trigger(timing::Post(), EndTurn()),
            listeners: x.listeners,
            entities: x.entities,
            storage: x.storage,
        }
    }
}

impl PullupFrom<Game<Trigger<timing::Post, EndTurn>>> for Game<Trigger<timing::Peri, EndTurn>> {
    fn pullup_from(x: Game<Trigger<timing::Post, EndTurn>>) -> Self {
        Game {
            state: Trigger(timing::Peri(), EndTurn()),
            listeners: x.listeners,
            entities: x.entities,
            storage: x.storage,
        }
    }
}
