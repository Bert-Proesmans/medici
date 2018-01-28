use std::fmt::Debug;

use containers::entities::EntityService;
// use containers::listeners::ListenerService;
use containers::tapes::TapeService;

use medici_macros::build_automaton;
use medici_traits::prelude::*;

/* DBG IMPLS */
impl<W: Waitable> Global for self::states::global::Wait<W> {}
impl<T: Timing, U: Actionable> Global for self::states::global::Action<T, U> {}
impl Global for self::states::global::Finished {}
impl<T: Timing, U: Triggerable> Global for self::states::global::Effect<T, U> {}
impl<T: Timing, U: Triggerable> Global for self::states::global::Trigger<T, U> {}
impl<T: Timing, U: Actionable> Global for self::states::global::Death<T, U> {}

impl Waitable for self::states::waitable::Input {}

impl Actionable for self::states::actionable::EndTurn {}
impl Triggerable for self::states::actionable::EndTurn {}

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
			#[derive(Debug)]
			struct Death<T: Timing, U: Actionable>(T, U);

			// Custom states can be defined below.
		}

		Waitable {
			#[derive(Debug)]
			struct Input();
		}

		// You could also re-export defaults from medici_traits!
		// Pre / Peri / Post for timing_states
		Timing {
			use medici_traits::timing::default::{Pre, Peri, Post};
			use medici_traits::timing::default::EnumerationTiming;
		}

		Actionable {
			#[derive(Debug)]
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

#[cfg(test)]
mod tests {
	use std::marker::PhantomData;
	use medici_traits::prelude::*;
	use medici_traits::automata::{PullupInto, PushdownInto};

	use super::*;
	use super::states::global::*;
	use super::states::waitable::*;
	use super::states::timing::*;
	use super::states::actionable::*;

	#[test]
	fn game_transitions() {
	    let game: Game<Wait<Input>> = Game {
	    	state: PhantomData,
			entities: EntityService::new(),
			storage: TapeService::new(),
	    };

	    let game: Game<Action<Pre, EndTurn>> = game.into();
	    let game: Game<Effect<Pre, EndTurn>> = game.pushdown();
	    let game: Game<Action<Pre, EndTurn>> = game.pullup();
	}
}

