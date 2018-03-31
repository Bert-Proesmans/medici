use marker::{Service, TransactionContainer};
use service::error::StackPopError;

/// Structure wrapping a [`Vec`] to provide a simple Stack interface.
#[derive(Debug, Clone)]
pub struct StackStorage<A>
where
    A: TransactionContainer,
{
    /// Backing storage for the emulated Stack functionality.
    pub tape: Vec<A>,
}

impl<A> Service for StackStorage<A>
where
    A: TransactionContainer,
{
}

impl<A> StackStorage<A>
where
    A: TransactionContainer,
{
    /// Creates a new object for storage.
    pub fn new() -> Self {
        Self { tape: vec![] }
    }

    /// Add the provided value onto the top of the Stack.
    pub fn push<T: Into<A>>(&mut self, t: T) -> Result<(), !> {
        self.tape.push(t.into());
        Ok(())
    }

    /// Remove the element from the top of the Stack.
    ///
    /// The popped value will match the value which was pushed last
    /// before executing this method.
    pub fn pop(&mut self) -> Result<A, StackPopError> {
        self.tape.pop().ok_or(StackPopError)
    }
}
