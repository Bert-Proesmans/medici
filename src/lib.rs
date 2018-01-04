pub mod global_states {
    #[derive(Debug)]
    pub struct Wait<W> {
        pub child: W,
    }

    #[derive(Debug)]
    pub struct Action<A> {
       pub child: A,
    }

    pub mod wait_states {
        pub struct Input();
    }

    pub mod action_states {
    	pub struct Pre<P>(P);
    	pub struct Post<P>(P);

        pub struct EndTurn();
    }
}

#[derive(Debug)]
pub struct Game<S> {
    state: S
}

use std::convert::From;
use global_states::*;
use global_states::wait_states::*;
use global_states::action_states::*;

impl From<Game<Wait<Input>>> for Action<Pre<EndTurn>> {
    fn from(x: Game<Wait<Input>>) -> Action<Pre<EndTurn>> {
    	Action {
    		child: Pre(EndTurn)
    	}
    }
}

fn entry() {}
