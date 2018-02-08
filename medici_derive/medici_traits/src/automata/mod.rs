pub mod pushdown_automaton;
pub mod deterministic_automaton;

pub trait State: Sized {
	type Transaction;
}

pub trait Transaction: Sized {}
