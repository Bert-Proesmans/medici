use automata::pushdown_automaton::{Pushdown, Pullup};
use containers::games::Game;
use hs_automaton::states::*;

impl Pushdown<Game<Effect<Pre, EndTurn>>> for Game<Action<Pre, EndTurn>> {
    fn pushdown(self) -> Game<Effect<Pre, EndTurn>> {
        Game {
            state: Effect(Pre(), EndTurn()),
            listeners: self.listeners,
            entities: self.entities,
            storage: self.storage,
        }
    }
}

impl Pullup<Game<Effect<Pre, EndTurn>>> for Game<Action<Pre, EndTurn>> {
    fn pullup(x: Game<Effect<Pre, EndTurn>>) -> Self {
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

impl Pushdown<Game<Trigger<Pre, EndTurn>>> for Game<Effect<Pre, EndTurn>> {
    fn pushdown(self) -> Game<Trigger<Pre, EndTurn>> {
        Game {
            state: Trigger(Pre(), EndTurn()),
            listeners: self.listeners,
            entities: self.entities,
            storage: self.storage,
        }
    }
}

impl Pullup<Game<Trigger<Pre, EndTurn>>> for Game<Effect<Pre, EndTurn>> {
    fn pullup(x: Game<Trigger<Pre, EndTurn>>) -> Self {
        Game {
            state: Effect(Pre(), EndTurn()),
            listeners: x.listeners,
            entities: x.entities,
            storage: x.storage,
        }
    }
}

impl Pushdown<Game<Trigger<Peri, EndTurn>>> for Game<Trigger<Pre, EndTurn>> {
    fn pushdown(self) -> Game<Trigger<Peri, EndTurn>> {
        Game {
            state: Trigger(Peri(), EndTurn()),
            listeners: self.listeners,
            entities: self.entities,
            storage: self.storage,
        }
    }
}

impl Pullup<Game<Trigger<Peri, EndTurn>>> for Game<Trigger<Pre, EndTurn>> {
    fn pullup(x: Game<Trigger<Peri, EndTurn>>) -> Self {
        Game {
            state: Trigger(Pre(), EndTurn()),
            listeners: x.listeners,
            entities: x.entities,
            storage: x.storage,
        }
    }
}

impl Pushdown<Game<Trigger<Post, EndTurn>>> for Game<Trigger<Peri, EndTurn>> {
    fn pushdown(self) -> Game<Trigger<Post, EndTurn>> {
        Game {
            state: Trigger(Post(), EndTurn()),
            listeners: self.listeners,
            entities: self.entities,
            storage: self.storage,
        }
    }
}

impl Pullup<Game<Trigger<Post, EndTurn>>> for Game<Trigger<Peri, EndTurn>> {
    fn pullup(x: Game<Trigger<Post, EndTurn>>) -> Self {
        Game {
            state: Trigger(Peri(), EndTurn()),
            listeners: x.listeners,
            entities: x.entities,
            storage: x.storage,
        }
    }
}
