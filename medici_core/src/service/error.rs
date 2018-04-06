//! Types for simplifying error handling syntax.

use std::fmt::{Debug, Display};

use failure::Fail;

use marker::ProtoEnumerator;

/// Specific error thrown when the [`StackStorage`] has no items left
/// and the users coded it to pop another item.
#[derive(Debug, Fail)]
#[fail(display = "Popped too many times!")]
pub struct StackPopError;

/// Specific error thrown to indicate the system cannot execute the request under
/// constrained circumstances.
#[derive(Debug, Fail)]
#[fail(display = "A constraint amount is overflowed, maximum is {:}", _0)]
pub struct OverflowError(pub usize);

/// Specific error thrown when the requested entity-id is not known.
#[derive(Debug, Fail)]
#[fail(display = "The entity with id `{:}` was not found", _0)]
pub struct MissingEntityError<ID: Display>(pub ID);

/// Specific error thrown when the requested entity-id is not known.
#[derive(Debug, Fail)]
#[fail(display = "The entity with id `{:}` doesn't have the prototype `{:?}`", _0, _1)]
pub struct MissingProtoTypeError<ID: Display, P: ProtoEnumerator + Debug>(pub ID, pub P);
