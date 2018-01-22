use medici_traits::prelude::*;

use containers::games::Game;
use hs_automaton::states::*;

impl From<Game<Wait<Input>>> for Game<Action<Pre, EndTurn>> {
    fn from(x: Game<Wait<Input>>) -> Self {
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

impl From<Game<Action<Pre, EndTurn>>> for Game<Death<Pre, EndTurn>> {
    fn from(x: Game<Action<Pre, EndTurn>>) -> Self {
        Game {
            state: Death(Pre(), EndTurn()),
            listeners: x.listeners,
            entities: x.entities,
            storage: x.storage,
        }
    }
}

impl From<Game<Death<Pre, EndTurn>>> for Game<Action<Post, EndTurn>> {
    fn from(x: Game<Death<Pre, EndTurn>>) -> Self {
        Game {
            state: Action {
                timing: Post(),
                activity: EndTurn(),
            },
            listeners: x.listeners,
            entities: x.entities,
            storage: x.storage,
        }
    }
}

impl From<Game<Action<Post, EndTurn>>> for Game<Death<Post, EndTurn>> {
    fn from(x: Game<Action<Post, EndTurn>>) -> Self {
        Game {
            state: Death(Post(), EndTurn()),
            listeners: x.listeners,
            entities: x.entities,
            storage: x.storage,
        }
    }
}

impl<U> From<Game<Death<Post, U>>> for Game<Wait<Input>>
where
    U: Actionable,
{
    fn from(x: Game<Death<Post, U>>) -> Self {
        Game {
            state: Wait { activity: Input() },
            listeners: x.listeners,
            entities: x.entities,
            storage: x.storage,
        }
    }
}

impl<T, U> From<Game<Death<T, U>>> for Game<Finished>
where
    T: Timing,
    U: Actionable,
{
    fn from(x: Game<Death<T, U>>) -> Game<Finished> {
        Game {
            state: Finished(),
            listeners: x.listeners,
            entities: x.entities,
            storage: x.storage,
        }
    }
}
