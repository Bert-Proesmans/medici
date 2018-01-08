// Build parent (self) from child (T)
pub trait Pullup<T>: Sized {
    fn pullup(_: T) -> Self;
}

// Build child (T) from parent (self)
pub trait Pushdown<T>: Sized {
    fn pushdown(self) -> T;
}

// impl<P, C> Pullup<C> for P
// where
//     // Parent implements pushdown into child
//     P: Pushdown<C>
// {
//     fn pullup(x: C) -> P {
//         // TODO
//     }
// }
