use std::fmt::Debug;

use containers::entities::EntityService;
// use containers::listeners::ListenerService;
use containers::tapes::TapeService;

use medici_macros::build_automaton;
use medici_traits::prelude::*;

build_automaton!{

	// Game object layout

	struct Game<X: Global> {
		// State MUST BE THE FIRST PARAMETER, defining X.
		// X represents on of the global states.
		state: X,
		// listeners: ListenerService,
		entities: EntityService,
		storage: TapeService,
	}

	states {
		// Each group represents a submodule (the names are converted to snake_case as well)
		Global {
			#[derive(Debug)]
			struct Wait<W: Waitable>(W);
			// Actionable inherits from Triggerable
			#[derive(Debug)]
			struct Action<T: Timing, U: Actionable>(T, U);
			#[derive(Debug)]
			struct Finished();

			#[derive(Debug)]
			struct Effect<T: Timing, U: Triggerable>(T, U);
			#[derive(Debug)]
			struct Trigger<T: Timing, U: Triggerable>(T, U);

			// Custom states can be defined below.
		}

		Waitable {
			struct Input();
		}

		// You could also re-export defaults from medici_traits!
		// Pre / Peri / Post for timing_states
		Timing {
			use medici_traits::timing::default::{Pre, Peri, Post};
			use medici_traits::timing::default::EnumerationTiming;
		}

		Actionable {
			struct EndTurn();
		}

		Triggerable {}
	}
	
	// Possible state machine transitions.

	// Into's are non-reversible transitions.
	// Pushdowns are reversible transitions, eg A <-> B will result
	// in a pushdown from A into B, and a pullup from B into A.
	transitions {
		// Game<X> is implicit and can be ommited, since it would make
		// the syntax more difficult to follow.
		into_transitions {
		 	Wait<Input> -> Action<Pre, EndTurn>,
			Death<Post, EndTurn> -> Wait<Input>,

			Action<Pre, EndTurn> -> Death<Pre, EndTurn>,
			Death<Pre, EndTurn> -> Action<Peri, EndTurn>,
			Action<Peri, EndTurn> -> Death<Peri, EndTurn>,
			Death<Peri, EndTurn> -> Action<Post, EndTurn>,
			Action<Post, EndTurn> -> Death<Post, EndTurn>,
		}

		pushdown_transitions {
			Action<Pre, EndTurn> <-> Effect<Pre, EndTurn>,
			Effect<Pre, EndTurn> <-> Trigger<Pre, EndTurn>,
			Trigger<Pre, EndTurn> <-> Trigger<Peri, EndTurn>,
			Trigger<Peri, EndTurn> <-> Trigger<Post, EndTurn>,
		}
	}
}

// #[cfg(test)]
mod tests {
	use medici_traits::prelude::*;

	use super::Game;

	// #[test]
	// fn game_struct() {
	//     let game = Game::new();
	// }
}

