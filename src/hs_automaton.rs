
// Containers must implement NoArgBuilder Trait!
// This allows for a new instance to be built WITHOUT any
// parameters.
// use containers::entities::EntityService;
// use containers::listeners::ListenerService;
// use containers::tapes::TapeService;

use medici_macros::build_automaton;

build_automaton!{

	// Game object layout

	Game {
		// State MUST BE THE FIRST PARAMETER, defining X.
		// X represents on of the global states.
		state: X,
		listeners: ListenerService,
		entities: EntityService,
		storage: TapeService,
	}

	// The following states must be omitted or provided in the exact order
	// as mentioned below!
	// You can rename the names of the states if you want to.
	global_states {
		Wait<Waitable>(),
		// Actionable inherits from Triggerable
		Action<Timing, Actionable: Triggerable>(),
		Finished(), // THIS STATE WILL BE USED AS FAILURE STATE

		Effect<Timing, Triggerable>(),
		Trigger<Timing, Triggerable>(),

		// Custom states can be defined below.
	}
}

/*
	// All states implementing the traits defined within global_states,
	// eg: Waitable -> waitable_states
	states {

		waitable_states {
			Input()
		}

		// Providing nothing means the defaults will be used;
		// Pre / Peri / Post for timing_states
		timing_states {}

		actionable_states {
			EndTurn()
		}

		triggerable_states {}
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
			Death<Pre, EndTurn> -> Action<Peri, EndTurn>>
			Action<Peri, EndTurn> -> Death<Peri, EndTurn>,
			Death<Peri, EndTurn> -> Action<Post, EndTurn>,
			Action<Post, EndTurn> -> Death<Post, EndTurn>,
		}

		pushdown_transitions {
			Action<Pre, EndTurn> <-> Effect<Pre, EndTurn>,
			Effect<Pre, EndTurn> <-> Trigger<Pre, EndTurn>,
			Trigger<Pre, EndTurn> <-> Trigger<Peri, EndTurn>,
			Trigger<Peri, EndTurn> <-> Trigger<Post, EndTurn>
		}
	}
}

*/

#[cfg(test)]
mod tests {
	use super::Game;

	#[test]
	fn game_struct() {
	    unimplemented!();
	}
}
