# Medici

> Provided examples are outdated. Please look at the code
> for usage examples.

[![Build Status](https://travis-ci.org/Bert-Proesmans/medici.svg?branch=master)](https://travis-ci.org/Bert-Proesmans/medici)
[![Docs](https://media.readthedocs.org/static/projects/badges/unknown.svg)](https://bert-proesmans.github.io/medici)

Ever searched for a "Game engine", but the results didn't satisfy your needs?  
Tried to update your search query with "Simulator engine" resulting in the same feeling?  
So have I, and decided to build Medici.

Medici is as much as a simulation engine as a game engine, but not following the same definitions
**everybody else uses**.
It DOES NOT come with tightly coupled event loops, input event handling, GUI or I/O systems.  
It's a framework which helps you easily build your own game simulator; processing input and while 
modifying the state of the simulator itself.  
Building your game with Medici will result in more loose coupling within your game's ecosystem.

## Principles

1 Maximum type safety;
	
	- No downcasting of generic objects required,
	- Transitions and States are linked through their type,
	- No guessing of current runtime state necessary,

2 Catch programming errors at compile time;

	- Prevents runtime errors before you even run tests,

3 Maximum implementation flexibilty;
	
	- Implement behaviours as triggers,
	- Freedom to hard-/dynamic code as much as you desire,
	- This leads to more consistently applied game rules,

4 Maximum performance.

## How to use it?

Medici is still in rapid development, but the outline is following;  

The actual simulation happens state-machine-wise, each transition is a change made 
to the system either through user action or a chained action (triggered).  
Each state fetches and executes listeners for occurred changes.  
The most important structure is the `Game` container containing all data generated 
during simulation. The type of this container represents the current state of 
the simulation.  
You model the state machine by calling the procedural macro `build_automaton`, 
filling in all information it requires.  
Now actual game coding can begin, implementing actions and behaviours.

## Examples

> This framework is still in rapid development and examples might become outdated.  
> Expect fundamental changes when Error propagation and default methods 
> (like `exec_action_listeners()`) are figured out.

### Generating state machine

> Reference: [src/automaton/config.rs]

The macro `build_automaton` builds everything for you, if all necessary information 
is provided.  
The intention is to create your simulator configuration in a seperate file, exporting
important pieces to your other code.
The traits you normally use are exported by the `medici_traits` crate, so make 
sure you import its prelude!  


```rust
#![feature(proc_macro)]

extern crate medici_macros;
use medici_macros::build_automaton;

extern crate medici_traits;
use medici_traits::prelude::*;

build_automaton!{
	// Declare the Game structure, defining containers for various purposes.
	// X is the generic parameter, implementing Global trait.
	struct Game<X: Global> {state: X}

	// Declare the Entity structure. This holds all information about anything within 
	// the game.
	// Most of the time you can leave this to be default.
	struct Entity {id: EntityId, state: HashMap<GameTags, u32>}

	// Declare the Card structure. Supposing your game has a concept of cards.
	struct Card {uid: CardId, name: &'static str, data: HashMap<GameTags, u32>}

	// Declare the set of states contained by the state machine.
	// Make sure to link Transactions to each state.
	// A Transaction is the object representing a transition, it forces you to pass down
	// 'this exact information' when changing states.
	// eg: Pass Entity ID of the card a user just played.
	states {
		// Declare states which are directly contained by the Game structure.
		Global {
			// Implement necessary GlobalState trait for Wait
			#[derive(GlobalState)]
			// Implement necessary State trait for Wait, providing a path to the Game 
			// structure and a path to the Transaction type (see below).
	        #[State(Game, transactions::Epsilon)]
	        // Implement the actual state, which can contain one a sub-state and/or 
	        // additionall state information.
	        // Wait represents a paused simulator, W indicates the necessary action to 
	        // continue.
	        struct Wait<W: Waitable>(W);

	        #[derive(Debug, GlobalState)]
	        #[State(Game, transactions::Epsilon)]
	        struct Trigger<T: Timing, U: Triggerable>(T, U);
		}

		Waitable {
	        #[derive(Debug, WaitState)]
	        struct Input();
	    }

		[..]
	}

	// All structures which are passed down when transitioning states.
	transactions {
        #[derive(Debug, Default)]
        // No transaction details are expected when using Epsilon.
        struct Epsilon();
    }

    // Declare all possible transitions.
    // eg: Game<Wait<Start>> -> Game<Action<Start>>
    transitions {
        // Game<X> is implicit and can be ommited, since it would make
        // the syntax more difficult to read.

        // Unidirectional transitions
        into_transitions {
            Wait<Start> -> Wait<Input>,
        }

        // Bi-directional transitions
        pushdown_transitions {
            Action<EndTurn> <-> Effect<EndTurn>,
        }
    }

    // Behaviours for our entities. Each entry will become a tuple struct
    // for readonly and read/write (postfix 'mut') access.
    prototypes {
        // Example of Game prototype:
        //      Game<'a>(&'a Entity);
        //      GameMut<'a>(&'a mut Entity);
        //
        // You can implement these prototypes in the implementations module.
        // The relevant Entity can be accessed by using self.0.
        Game,
    }
}
```

### Implementing EndTurn action

> Reference: [src/automaton/implementations/effects/actions.rs]

This example shows how to bootstrap the EndTurn action.

According to our automaton configuration the simulator pauses on `Game<Wait<Input>>`.  
To initiate our action we have to transition into `Game<Action<EndTurn>>`, after which we 
trigger all listeners for that specific state.  
Note that methods consuming a simulation state ALWAYS return the same state when no errors
are encountered.  
Errors have not been properly worked out at this moment so `Game<Finished>` is returned.

```rust
use medici_traits::automata::deterministic_automaton::TransitionInto;
use medici_traits::automata::pushdown_automaton::{PullupInto, PushdownInto};

use automaton::prelude::*;
use automaton::runtime::exec_action_listeners;

// Method invoked by user action: EndTurn
pub fn end_turn(x: Game<Wait<Input>>) -> Result<Game<Wait<Input>>, Game<Finished>> {
	// Transition into the desired state.
    let action: Game<Action<EndTurn>> = x.transition(Epsilon());
    // Execute all listeners for this action.
    let effect = exec_action_listeners(action.pushdown())?;

    // Circle back towards the type we received..
    let post_action: Game<Action<EndTurn>> = effect.pullup();
    // .. and return the requested state.
    Ok(post_action.transition(Epsilon()))
}
```

## Helping out

Any help offered is gladly accepted; specifically usability design, implementation and
use cases.

* Usability design

	- The macro `build_automaton` is a big mess currently and effort must be put into
	reducing the time to comprehend what its components do.
	- A lot of code can be seen as boilerplate, like `exec_action_listeners`. The main
	blocker here is that most boilerplate code actually needs to know the actual Game
	structure and other components built through `build_automaton`.  
	This could (maybe) be solved by utilizing declarative macros!?

* Use cases

	- The framework is currently being built in a way that prefers board-/card game
	implementations. Coming up with more fundamentally different game designs could
	prevent the framework from locking into this specific use case.
