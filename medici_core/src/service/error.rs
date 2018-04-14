//! Types for simplifying error handling syntax.

use std::fmt::{self, Debug, Display};

use failure::Fail as FailTrait;
use failure_derive::Fail;

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

/*
 * Code below contains a workaround for a pending failure_derive bug.
 * Check the toplevel module [`workaround`] for more information.
 */

/// Specific error thrown when the requested entity-id is not known.
#[derive(Debug)]
// #[fail(display = "The entity with id `{:}` was not found", _0)]
pub struct MissingEntityError<ID>(pub ID)
where
    ID: Display + Debug;

impl<ID> FailTrait for MissingEntityError<ID>
where
    ID: Display + Debug + Send + Sync + 'static,
{
}

impl<ID> fmt::Display for MissingEntityError<ID>
where
    ID: Display + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The entity with id `{:}` was not found", self.0)
    }
}

/// Specific error thrown when the requested entity-id is not known.
#[derive(Debug)]
// #[fail(display = "The entity with id `{:}` doesn't have the prototype `{:?}`", _0, _1)]
pub struct MissingPrototypeError<ID, P>(pub ID, pub P)
where
    ID: Display + Debug,
    P: ProtoEnumerator + Debug;

impl<ID, P> FailTrait for MissingPrototypeError<ID, P>
where
    ID: Display + Debug + Send + Sync + 'static,
    P: ProtoEnumerator + Debug + Send + Sync + 'static,
{
}

impl<ID, P> fmt::Display for MissingPrototypeError<ID, P>
where
    ID: Display + Debug,
    P: ProtoEnumerator + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "The entity with id `{:}` doesn't have the prototype `{:?}`",
            self.0, self.1
        )
    }
}
