//! Module which implements a CONS-LIST for transition validation at compile
//! time.

use std::marker::PhantomData;

/// Helper for creating a type that has a size of 0.
pub type Zero<T> = PhantomData<T>;

/// Empty stack on the go!
pub type Empty = Zero<()>;

/// Uninstantiable type to fail compilation on illegal stack operation.
pub enum Terminal {}

/// Traits facilitating implementation of a compile-time stack.
///
/// This trait concept is used to enforce correct state machine transitions.
pub trait Stack {
    /// Type of the front of the stack.
    type Head;
    /// Type of the next to front type on the stack.
    type Tail: Stack;
}

#[macro_export]
macro_rules! combine {
    ($stack:ty , $new_item:ty) => {
        Zero<($stack, $new_item)>
    }
}

impl Stack for Terminal {
    type Head = Terminal;
    type Tail = Terminal;
}

impl Stack for () {
    type Head = Terminal;
    type Tail = Terminal;
}

impl<T: Stack> Stack for Zero<T> {
    type Head = T::Head;
    type Tail = Zero<T::Tail>;
}

impl<S, X> Stack for (S, X)
where
    S: Stack,
{
    type Head = X;
    type Tail = S;
}
