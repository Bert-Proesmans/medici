use std::marker::Sized;

pub trait PullupFrom<T> : Sized + super::State
where
    T: Sized,
{
    fn pullup_from(_: T) -> Self;
}

pub trait PullupInto<T> : Sized
where
    T: Sized + super::State,
{
    fn pullup(self) -> T;
}

impl<T, U> PullupInto<U> for T
where
    U: PullupFrom<T>,
{
    fn pullup(self) -> U {
        U::pullup_from(self)
    }
}

pub trait PushdownFrom<T> : Sized + super::State
where
    T: Sized,
{
    fn pushdown_from(_: T) -> Self;
}

pub trait PushdownInto<T> : Sized
where
    T: Sized + super::State,
{
    fn pushdown(self) -> T;
}

impl<T, U> PushdownInto<U> for T
where
    U: PushdownFrom<T>,
{
    fn pushdown(self) -> U {
        U::pushdown_from(self)
    }
}
