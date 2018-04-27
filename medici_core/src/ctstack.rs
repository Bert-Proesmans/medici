//! Module which implements a CONS-LIST for transition validation at compile
//! time.

use std::marker::PhantomData;

use function::State;
use marker;

/// Re-exported because standard library module (std) is private.
/// Consequentially macros cannot directly use std types which are not
/// inside the std-prelude.
pub type ZeroSizedType<T> = PhantomData<T>;

/// Type for starting a new CTStack.
pub type EmptyStack = ();
/// Type representing aq CTStack with any contents.
///
/// # Safety
/// This is valid if all CTStack types are zero-sized!
/// Do NOT use this type otherwise.
pub type AnyStack = ();

/// Usability macro for pushing a new type onto the CTStack.
#[macro_export]
macro_rules! ct {
    // Push new item onto the provided stack.
    ($new_item:ty => $stack:ty) => {
        ($stack, $crate::ctstack::ZeroSizedType<$new_item>)
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

// Not wrapped into PhantomData because '()' is already a zero sized type.
impl CTStack for () {
    type Head = ();
    type Tail = !;
}

// X wrapped in PhantomData to make it zero-sized.
impl<X> CTStack for (PhantomData<X>,)
where
    X: State + marker::TopLevel,
{
    type Head = X;
    type Tail = ();
}

// X wrapped in PhantomData to make it zero-sized.
// This implementation supposes that S is already a zero-sized stack.
impl<S, X> CTStack for (S, PhantomData<X>)
where
    S: CTStack,
    X: State + marker::TopLevel,
{
    type Head = X;
    type Tail = S;
}
