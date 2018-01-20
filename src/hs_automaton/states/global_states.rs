use medici_traits::prelude::*;

pub trait GlobalState {}

#[derive(Debug)]
pub struct Wait<W>
where
    W: Waitable,
{
    pub activity: W,
}

#[derive(Debug)]
pub struct Action<T, A>
where
    T: Timing,
    A: Actionable,
{
    pub timing: T,
    pub activity: A,
}

#[derive(Debug)] // , State
pub struct Finished();
