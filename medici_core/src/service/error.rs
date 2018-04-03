//! Types for simplifying error handling syntax.

use failure::Fail;

use function::EntityId;

/// Specific error thrown when the [`StackStorage`] has no items left
/// and the users coded it to pop another item.
#[derive(Debug, Fail)]
#[fail(display = "Popped too many times!")]
pub struct StackPopError;

/// Specific error thrown to indicate the system cannot execute the request under
/// constrained circumstances.
#[derive(Debug, Fail)]
#[fail(display = "A constraint amount is overflowed, maximum is {:?}", _0)]
pub struct OverflowError(pub usize);

/// Specific errors thrown related to Entities in the state machine.
/// One of the users of this enum is [`EntityStorage`].
#[derive(Debug, Fail)] // Copy, Clone, Eq, PartialEq
pub enum EntityError {
    /// The requested entity does not exist within the machine.
    #[fail(display = "Entity {:?} not found", _0)]
    MissingEntityError(EntityId),
}
