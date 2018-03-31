//! Types for simplifying error handling syntax.

/// Specific error thrown when the [`StackStorage`] has no items left
/// and the users coded it to pop another item.
#[derive(Debug, Fail)]
#[fail(display = "Popped too many times!")]
pub struct StackPopError;
