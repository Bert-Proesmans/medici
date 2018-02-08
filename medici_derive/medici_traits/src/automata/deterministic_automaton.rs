use std::marker::Sized;

pub trait TransitionFrom<T>: Sized + super::State
where
	T: Sized,
	Self::Transaction: super::Transaction,
{
    fn transition_from(_: T, _: Self::Transaction) -> Self;
}

pub trait TransitionInto<T> : Sized
where
	T: Sized + super::State,
	T::Transaction: super::Transaction,
{
	fn transition(self, _: T::Transaction) -> T;
}

impl<T, U> TransitionInto<U> for T 
where
	U: TransitionFrom<T>,
	U::Transaction: super::Transaction,
{
    fn transition(self, transaction: U::Transaction) -> U {
    	U::transition_from(self, transaction)
    }
}
