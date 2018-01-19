use medici_traits::pushdown_automaton::{PullupFrom, PushdownFrom};

use containers::games::Game;
use hs_automaton::states::*;

impl PushdownFrom<Game<Action<Pre, EndTurn>>> for Game<Effect<Pre, EndTurn>> {
    fn pushdown_from(x: Game<Action<Pre, EndTurn>>) -> Self {
        Game {
            state: Effect(Pre(), EndTurn()),
            listeners: x.listeners,
            entities: x.entities,
            storage: x.storage,
        }
    }
}

impl PullupFrom<Game<Effect<Pre, EndTurn>>> for Game<Action<Pre, EndTurn>> {
    fn pullup_from(x: Game<Effect<Pre, EndTurn>>) -> Self {
        Game {
            state: Action {
                timing: Pre(),
                activity: EndTurn(),
            },
            listeners: x.listeners,
            entities: x.entities,
            storage: x.storage,
        }
    }
}

impl PushdownFrom<Game<Effect<Pre, EndTurn>>> for Game<Trigger<Pre, EndTurn>> {
    fn pushdown_from(x: Game<Effect<Pre, EndTurn>>) -> Self {
        Game {
            state: Trigger(Pre(), EndTurn()),
            listeners: x.listeners,
            entities: x.entities,
            storage: x.storage,
        }
    }
}

impl PullupFrom<Game<Trigger<Pre, EndTurn>>> for Game<Effect<Pre, EndTurn>> {
    fn pullup_from(x: Game<Trigger<Pre, EndTurn>>) -> Self {
        Game {
            state: Effect(Pre(), EndTurn()),
            listeners: x.listeners,
            entities: x.entities,
            storage: x.storage,
        }
    }
}

impl PushdownFrom<Game<Trigger<Pre, EndTurn>>> for Game<Trigger<Peri, EndTurn>> {
    fn pushdown_from(x: Game<Trigger<Pre, EndTurn>>) -> Self {
        Game {
            state: Trigger(Peri(), EndTurn()),
            listeners: x.listeners,
            entities: x.entities,
            storage: x.storage,
        }
    }
}

impl PullupFrom<Game<Trigger<Peri, EndTurn>>> for Game<Trigger<Pre, EndTurn>> {
    fn pullup_from(x: Game<Trigger<Peri, EndTurn>>) -> Self {
        Game {
            state: Trigger(Pre(), EndTurn()),
            listeners: x.listeners,
            entities: x.entities,
            storage: x.storage,
        }
    }
}

impl PushdownFrom<Game<Trigger<Peri, EndTurn>>> for Game<Trigger<Post, EndTurn>> {
    fn pushdown_from(x: Game<Trigger<Peri, EndTurn>>) -> Self {
        Game {
            state: Trigger(Post(), EndTurn()),
            listeners: x.listeners,
            entities: x.entities,
            storage: x.storage,
        }
    }
}

impl PullupFrom<Game<Trigger<Post, EndTurn>>> for Game<Trigger<Peri, EndTurn>> {
    fn pullup_from(x: Game<Trigger<Post, EndTurn>>) -> Self {
        Game {
            state: Trigger(Peri(), EndTurn()),
            listeners: x.listeners,
            entities: x.entities,
            storage: x.storage,
        }
    }
}
