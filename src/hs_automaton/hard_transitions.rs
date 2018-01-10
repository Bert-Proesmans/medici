use containers::games::Game;
use hs_automaton::states::*;
use hs_automaton::states::global_states::timing;

impl From<Game<Wait<Input>>> for Game<Action<timing::Pre, EndTurn>> {
    fn from(x: Game<Wait<Input>>) -> Self {
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

impl From<Game<Action<timing::Pre, EndTurn>>> for Game<Death<timing::Pre, EndTurn>> {
    fn from(x: Game<Action<timing::Pre, EndTurn>>) -> Self {
        Game {
            state: Death(timing::Pre(), EndTurn()),
            listeners: x.listeners,
            entities: x.entities,
            storage: x.storage,
        }
    }
}

impl From<Game<Death<timing::Pre, EndTurn>>> for Game<Action<timing::Post, EndTurn>> {
    fn from(x: Game<Death<timing::Pre, EndTurn>>) -> Self {
        Game {
            state: Action {
                timing: timing::Post(),
                activity: EndTurn(),
            },
            listeners: x.listeners,
            entities: x.entities,
            storage: x.storage,
        }
    }
}

impl From<Game<Action<timing::Post, EndTurn>>> for Game<Death<timing::Post, EndTurn>> {
    fn from(x: Game<Action<timing::Post, EndTurn>>) -> Self {
        Game {
            state: Death(timing::Post(), EndTurn()),
            listeners: x.listeners,
            entities: x.entities,
            storage: x.storage,
        }
    }
}

impl<U> From<Game<Death<timing::Post, U>>> for Game<Wait<Input>> {
    fn from(x: Game<Death<timing::Post, U>>) -> Self {
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
    T: timing::Timing,
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
