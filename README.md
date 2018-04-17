# Medici

[![Build Status](https://travis-ci.org/Bert-Proesmans/medici.svg?branch=master)](https://travis-ci.org/Bert-Proesmans/medici)
[![Docs](https://media.readthedocs.org/static/projects/badges/unknown.svg)](https://bert-proesmans.github.io/medici)

Ever searched for a "Game engine", but the results didn't satisfy your needs?  
Tried to update your search query with "Simulator engine" resulting in the same feeling?  
So have I, and decided to build Medici.

Medici is an opiniated server simulation framework.
It DOES NOT come with tightly coupled event loops, user input event handling, GUI or audio systems.  
It DOES help you easily build your own (board) game simulator in a robust and typesafe manner.  
Medici and derivates are meant to be used as a building block, but remain flexible. Other libraries could wrap
the game's state machine or composite them into the state machine itself. The latter allows for these libraries
to be easily used dynamically during the game.

## Principles

1 Maximum type safety;
	
	- Any dynamic code can be hardcoded to their trigger condition.
	- No guessing of current runtime state necessary,	
	- Leads to maximum robustness,

2 Catch programming errors at compile time;

	- Transitions are fixed to chosen State types,
	- The type system validates ALL state transitions at compile time,

3 Maximum implementation flexibilty;
	
	- Implement behaviours as triggers*,
	- Freedom to implement dynamic behaviour in a loosly coupled manner,

4 Maximum performance.

> *A trigger is a function which is registered with - and stored within the game's state machine.
> A trigger function is coupled to one 'triggerable' state and one 'timing' state. Both these states
> are used as condition for when to execute the trigger (function).

## How to use it?

> Medici is still being worked on. Mainly usability improvements are still under construction.
> It's concepts however are already clear.  

> Look at [game_system] and [game_rules] for an example implementation.

Use Medici concepts to build a state machine for your game. All relevant data and components are
stored within this state machine, so the term game container will be used interchangably.
Next up is to define all states which your game could transition into.
With all states known you write transitions. Two transition concepts are supported; one-way
transitions and bi-directional transitions (Pushdown+Pullup).
All of the above should be stored into one crate which you'd describe as container for your 
game system(s).  
Next up, contained by another crate, is implementation of game triggers, player actions and game-cards.
These components are the embodiment of your game rules.  
Now you have completely implemented your game system and -rules. Combine these crates with other
rust crates to handle networked input, logging, time constraints etc. Whatever fits your use case.

## Helping out

Any help offered is gladly accepted; specifically for usability design, implementation and
use cases.

* Usability design

	- A lot of code within game_system can be seen as boilerplate. Macros could be used
	to automatically generate all boilerplate. The main	blocker here is that most boilerplate code 
	actually needs to know all the fields of the game structure. This problem could be solved
	with compiler extensions, probably.

* Use cases

	- The framework is currently being built in a way that favors board-/card game
	implementations. Coming up with more fundamentally different game designs prevents us, developers,
	from tunnel vision. Tunnel vision is dangerous because the framework will be too heavily optimized
	for board specific games.
