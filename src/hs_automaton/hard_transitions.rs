use containers::game::Game;
use hs_automaton::states::*;

impl From<Game<Wait<Input>>> for Game<Action<Pre, EndTurn>> {
    fn from(_x: Game<Wait<Input>>) -> Self {
        Game {
            state: Action {
                timing: Pre(),
                activity: EndTurn(),
            },
        }
    }
}

impl From<Game<Action<Pre, EndTurn>>> for Game<Death<Pre, EndTurn>> {
    fn from(_x: Game<Action<Pre, EndTurn>>) -> Self {
        Game { state: Death(Pre(), EndTurn()) }
    }
}

impl From<Game<Death<Pre, EndTurn>>> for Game<Action<Post, EndTurn>> {
    fn from(_x: Game<Death<Pre, EndTurn>>) -> Self {
        Game {
            state: Action {
                timing: Post(),
                activity: EndTurn(),
            },
        }
    }
}

impl From<Game<Action<Post, EndTurn>>> for Game<Death<Post, EndTurn>> {
    fn from(_x: Game<Action<Post, EndTurn>>) -> Self {
        Game { state: Death(Post(), EndTurn()) }
    }
}

impl<U> From<Game<Death<Post, U>>> for Game<Wait<Input>> {
    fn from(_x: Game<Death<Post, U>>) -> Self {
        Game { state: Wait { activity: Input() } }
    }
}

impl<T, U> From<Game<Death<T, U>>> for Game<Finished> {
    fn from(_x: Game<Death<T, U>>) -> Game<Finished> {
        Game { state: Finished() }
    }
}
