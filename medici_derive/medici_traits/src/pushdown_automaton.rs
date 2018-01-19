pub trait PullupFrom<T>: Sized {
    fn pullup_from(_: T) -> Self;
}

pub trait PullupInto<T> {
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

pub trait PushdownFrom<T>: Sized {
    fn pushdown_from(_: T) -> Self;
}

pub trait PushdownInto<T> {
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
