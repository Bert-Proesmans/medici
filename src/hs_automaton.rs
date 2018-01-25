use std::fmt::Debug;

use containers::entities::EntityService;
// use containers::listeners::ListenerService;
use containers::tapes::TapeService;

use medici_macros::{build_automaton, GlobalState};

build_automaton!{

	// Game object layout

	Game {
		// State MUST BE THE FIRST PARAMETER, defining X.
		// X represents on of the global states.
		state: X,
		// listeners: ListenerService,
		entities: EntityService,
		storage: TapeService,
	}

	// The following states must be omitted or provided in the exact order
	// as mentioned below!
	// You can rename the names of the states if you want to.
	global_states {
		#[derive(Debug)]
		Wait<W: Waitable>(W),
		// Actionable inherits from Triggerable
		#[derive(Debug)]
		Action<T: Timing, U: Actionable>(T, U),
		#[derive(Debug)]
		Finished(), // THIS STATE WILL BE USED AS FAILURE STATE

		#[derive(Debug)]
		Effect<T: Timing, U: Triggerable>(T, U),
		#[derive(Debug)]
		Trigger<T: Timing, U: Triggerable>(T, U),

		// Custom states can be defined below.
	}

	states {
		// Each group identifier represents a procedural derive macro which
		// implements the desired trait for each of the states contained by that group
		// eg: Waitable { Input() } becomes 
		// 		#[derive(Debug, Waitable)]
		// 		pub struct Input();
		Waitable {
			Input()
		}

		// Providing nothing means the defaults will be used;
		// Pre / Peri / Post for timing_states
		Timing {}

		Actionable {
			EndTurn()
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
			Action<Pre, EndTurn> <-> Effect<Pre, EndTurn>
			Effect<Pre, EndTurn> <-> Trigger<Pre, EndTurn>
			Trigger<Pre, EndTurn> <-> Trigger<Peri, EndTurn>
			Trigger<Peri, EndTurn> <-> Trigger<Post, EndTurn>
		}
	}
}

#[cfg(test)]
mod tests {
	use medici_traits::prelude::*;

	use super::{TapeService, EntityService, Game};
	use super::states::global::{Wait, Finished};

	// impl Global for Finished {}

	impl Game<Finished> {
	    fn new() -> Self {
	    	Game {
	    		state: Finished(),
	    		entities: EntityService::new(),
				storage: TapeService::new(),
	    	}
	    }
	}	

	#[test]
	fn game_struct() {
	    let game = Game::new();
	}
}
