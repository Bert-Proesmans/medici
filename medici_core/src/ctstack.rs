//! Module which implements a CONS-LIST for transition validation at compile
//! time.

use std::fmt::Debug;

use function::State;
use marker;

/// Type for starting a new CTStack.
pub type EmptyStack = ();
/// Type representing aq CTStack with any contents.
///
/// # Safety
/// This is only valid when the implemented size is 0.
pub type AnyStack = ();

/// Usability macro for pushing a new type onto the CTStack.
#[macro_export]
macro_rules! ct {
    // Push new item onto the provided stack.
    ($new_item:ty => $stack:ty) => {
        ($stack, $new_item)
    };
}

/// Traits facilitating implementation of a compile-time stack.
///
/// This trait concept is used to enforce correct state machine transitions.
pub trait CTStack {
    /// Type of the front of the stack.
    type Head;
    /// Type of the next to front type on the stack.
    type Tail: CTStack;
}

impl CTStack for ! {
    type Head = !;
    type Tail = !;
}

impl CTStack for () {
    type Head = ();
    type Tail = !;
}

impl<X> CTStack for (X,)
where
    X: State + marker::TopLevel,
{
    type Head = X;
    type Tail = ();
}

impl<S, X> CTStack for (S, X)
where
    S: CTStack,
    X: State + marker::TopLevel,
{
    type Head = X;
    type Tail = S;
}
