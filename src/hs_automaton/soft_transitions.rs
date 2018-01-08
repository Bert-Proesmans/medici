use automata::pushdown_automaton::{Pushdown, Pullup};
use containers::game::Game;
use hs_automaton::states::*;

impl Pushdown<Game<Effect<Pre, EndTurn>>> for Game<Action<Pre, EndTurn>> {
    fn pushdown(self) -> Game<Effect<Pre, EndTurn>> {
        Game { state: Effect(Pre(), EndTurn()) }
    }
}

impl Pullup<Game<Effect<Pre, EndTurn>>> for Game<Action<Pre, EndTurn>> {
    fn pullup(_x: Game<Effect<Pre, EndTurn>>) -> Self {
        Game {
            state: Action {
                timing: Pre(),
                activity: EndTurn(),
            },
        }
    }
}

impl Pushdown<Game<Trigger<Pre, EndTurn>>> for Game<Effect<Pre, EndTurn>> {
    fn pushdown(self) -> Game<Trigger<Pre, EndTurn>> {
        Game { state: Trigger(Pre(), EndTurn()) }
    }
}

impl Pullup<Game<Trigger<Pre, EndTurn>>> for Game<Effect<Pre, EndTurn>> {
    fn pullup(_x: Game<Trigger<Pre, EndTurn>>) -> Self {
        Game { state: Effect(Pre(), EndTurn()) }
    }
}

impl Pushdown<Game<Trigger<Peri, EndTurn>>> for Game<Trigger<Pre, EndTurn>> {
    fn pushdown(self) -> Game<Trigger<Peri, EndTurn>> {
        Game { state: Trigger(Peri(), EndTurn()) }
    }
}

impl Pullup<Game<Trigger<Peri, EndTurn>>> for Game<Trigger<Pre, EndTurn>> {
    fn pullup(_x: Game<Trigger<Peri, EndTurn>>) -> Self {
        Game { state: Trigger(Pre(), EndTurn()) }
    }
}

impl Pushdown<Game<Trigger<Post, EndTurn>>> for Game<Trigger<Peri, EndTurn>> {
    fn pushdown(self) -> Game<Trigger<Post, EndTurn>> {
        Game { state: Trigger(Post(), EndTurn()) }
    }
}

impl Pullup<Game<Trigger<Post, EndTurn>>> for Game<Trigger<Peri, EndTurn>> {
    fn pullup(_x: Game<Trigger<Post, EndTurn>>) -> Self {
        Game { state: Trigger(Peri(), EndTurn()) }
    }
}
