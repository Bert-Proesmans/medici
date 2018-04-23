//! Types, to be used within the system, providing context of unexpected behaviour.

use std::fmt::{self, Debug, Display, Formatter};
use std::string::ToString;

use failure::{Backtrace, Context, Fail};

use function::StateContainer;

/// User facing error type indicating an issue ocurred during evalutation of any
/// state machine processes.
///
/// This structure should be used to create an error that is presented to the end-user
/// or external systems. It carries a snapshot of the state-machine at the moment
/// the error occurred.
#[derive(Debug)]
pub struct MachineError {
    machine: Box<(Debug + Send + Sync)>,
    inner: Context<ErrorKind>,
}

impl Fail for MachineError {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for MachineError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

/// Enumeration of publicl cases of state machine failures.
#[derive(Debug, Fail, Copy, Clone, Eq, PartialEq)]
pub enum ErrorKind {
    /// Error indicating some constraint failed to assert at runtime.
    #[fail(display = "An operation failed to meet required constraints")]
    ConstraintError,
    /// Error indicating the developer has introduced a logic error in
    /// his code.
    #[fail(display = "A logical error ocurred")]
    LogicError,
}

/// Trait facilitating error creation with a snapshot of the state machine
/// attached.
pub trait SnapshottedErrorExt<T> {
    /// Builds a [`MachineError`] from some error or empty option.
    ///
    /// # Constraints
    /// The error in question MUST implement [`Fail`]!
    ///
    /// # Parameters
    /// context [`ErrorKind`] - is ment to categorize different errors. Make sure the value
    /// you choose is semantically correct because that's all the communicated information
    /// to the end user.
    /// machine [´StateContainer`] - is ment to store (effectively through [`Clone`]) a
    /// snapshot of the state machine onto the heap. The stored state machine will be an exact
    /// copy of the real one at the moment of failure.
    fn context<M>(self, context: ErrorKind, machine: &M) -> Result<T, MachineError>
    where
        M: StateContainer + Clone + Debug + Sync + Send + 'static;
}

impl<T, E> SnapshottedErrorExt<T> for Result<T, E>
where
    E: Fail,
{
    fn context<M>(self, context: ErrorKind, machine: &M) -> Result<T, MachineError>
    where
        M: StateContainer + Clone + Debug + Sync + Send + 'static,
    {
        self.map_err(move |failure| {
            // Build and return custom error type
            MachineError {
                machine: Box::new(machine.clone()),
                // Build new context for our own error kind.
                // and chain the previous one..
                inner: failure.context(context),
            }
        })
    }
}

impl<T> SnapshottedErrorExt<T> for Option<T> {
    fn context<M>(self, context: ErrorKind, machine: &M) -> Result<T, MachineError>
    where
        M: StateContainer + Clone + Debug + Sync + Send + 'static,
    {
        match self {
            Some(v) => Ok(v),
            None => {
                // Build and return custom error type
                Err(MachineError {
                    machine: Box::new(machine.clone()),
                    // Build new context for our own error kind.
                    // and chain the previous one..
                    inner: Context::new(context),
                })
            }
        }
    }
}

/// Type used for indicating failure to meet specified constraints.
#[derive(Debug, Fail)]
#[fail(display = "Constraint violation detected! Expected `{:}`, provided `{:}`", expected, factual)]
pub struct RuntimeConstraintError {
    /// Value defining the constraint.
    expected: String,
    /// Value which fails to meet the constraint.
    factual: String,
}

impl<S1, S2> From<(S1, S2)> for RuntimeConstraintError
where
    S1: ToString,
    S2: ToString,
{
    fn from(x: (S1, S2)) -> Self {
        let (expected, factual) = x;
        RuntimeConstraintError {
            expected: expected.to_string(),
            factual: factual.to_string(),
        }
    }
}

/// Code failed to push a new item onto the chosen stack.
#[derive(Debug, Fail)]
#[fail(display = "Error pushing data to the stack")]
pub struct StackPushError {}
