//! Types, to be used within the system, providing context of unexpected behaviour.

use std::borrow::Cow;
use std::fmt::{self, Debug, Display, Formatter};
use std::string::ToString;
use std::sync::{Arc, Mutex};

use failure::{Backtrace, Context, Error, Fail};

use function::StateContainer;
use marker;

/// User facing error type indicating a failure during evalutation/computation of the
/// state machine.
///
/// This structure should be used to create an error that is presented to the end-user
/// or external systems. It carries a snapshot of the state-machine at the earliest moment
/// after the failure occurred.
#[derive(Debug)]
pub struct MachineError {
    // Debug + Send becomes Debug + Send + Sync when wrapped in the
    // Arc<Mutex<_>> combo!
    machine: Arc<Mutex<Debug + Send>>,
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
#[derive(Debug, Fail, Clone, Eq, PartialEq)]
pub enum ErrorKind {
    /// Error indicating some constraint failed to assert at runtime.
    #[fail(display = "An operation failed to meet required constraints")]
    ConstraintError,
    /// Error indicating the developer has introduced a logic error in
    /// his code.
    #[fail(display = "A logical error ocurred")]
    LogicError,
    /// Error allowing to print an entirely custom message
    #[fail(display = "{:}", _0)]
    Custom(Cow<'static, str>),
}

/// TODO
pub trait HydratedErrorExt {
    /// TODO
    fn hydrate<M, I>(self, machine: I) -> MachineError
    where
        I: FnOnce() -> M,
        M: StateContainer + Debug + Send + 'static;
}

impl HydratedErrorExt for Error {
    fn hydrate<M, I>(self, machine: I) -> MachineError
    where
        I: FnOnce() -> M,
        M: StateContainer + Debug + Send + 'static,
    {
        MachineError {
            machine: Arc::new(Mutex::new((machine)())),
            // TODO; Figure out how to reach the original error and
            // build a context from there.
            inner: self.context(ErrorKind::LogicError),
        }
    }
}

/// Trait facilitating error creation with a snapshot of the state machine
/// attached.
pub trait FrontendErrorExt {
    /// Builds a [`MachineError`] from some error or empty option.
    ///
    /// # Constraints
    /// The error in question MUST implement [`Fail`]!
    ///
    /// # Parameters
    /// context [`ErrorKind`] - is ment to categorize different errors. Make sure the value
    /// you choose is semantically correct because that's all the communicated information
    /// to the end user.
    ///
    /// machine [´StateContainer`] - is ment to store (effectively through [`Clone`]) a
    /// snapshot of the state machine onto the heap. The stored state machine will be an exact
    /// copy of the real one at the moment of failure.
    fn infuse<M, I>(self, context: ErrorKind, machine: I) -> MachineError
    where
        I: FnOnce() -> M,
        M: StateContainer + Debug + Send + 'static;

    /// Builds a [`MachineError`] from some error or empty option. The [`ErrorKind`] invariant
    /// will be [`ErrorKind::Custom`] which will contain the string type passed into this method.
    ///
    /// # Parameters
    /// custom [`Cow`] - represents an owned string or a string reference.
    ///
    /// machine [´StateContainer`] - is ment to store (effectively through [`Clone`]) a
    /// snapshot of the state machine onto the heap. The stored state machine will be an exact
    /// copy of the real one at the moment of failure.
    fn infuse_with<F, M>(self, msg: F, machine: M) -> MachineError
    where
        F: Into<Cow<'static, str>>,
        M: StateContainer + Debug + Send + 'static;
}

impl<E> FrontendErrorExt for E
where
    E: Fail,
{
    fn infuse<M, I>(self, context: ErrorKind, machine: I) -> MachineError
    where
        I: FnOnce() -> M,
        M: StateContainer + Debug + Send + 'static,
    {
        // Build and return custom error type
        MachineError {
            machine: Arc::new(Mutex::new((machine)())),
            // Build new context for our own error kind.
            // and chain the previous one..
            inner: self.context(context),
        }
    }

    fn infuse_with<F, M>(self, msg: F, machine: M) -> MachineError
    where
        F: Into<Cow<'static, str>>,
        M: StateContainer + Debug + Send + 'static,
    {
        // Build and return custom error type
        MachineError {
            machine: Arc::new(Mutex::new(machine)),
            // Build new context for our own error kind.
            // and chain the previous one..
            inner: self.context(ErrorKind::Custom(msg.into())),
        }
    }
}

/// Error types which represent one specific kind of error (failure).
/// These errors are reported within functions that manipulate properties of the state machine,
/// but will be wrapped into [`MachineError`] eventually.
pub mod custom_type {
    use super::*;

    /// Type used for indicating failure to meet specified constraints.
    #[derive(Debug, Fail)]
    #[fail(
        display = "Constraint violation detected! Expected `{:}`, provided `{:}`", expected, factual
    )]
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

    /// Enumeration of publicl cases of state machine failures.
    #[derive(Debug, Fail, Clone, Eq, PartialEq)]
    pub enum ZoneMoveError {
        /// Error indicating that the entity was not found in a zone.
        #[fail(display = "The entity was not found within the provided zone")]
        NotInZone,
        /// Error indicating that the targetted zone is already at maximum capacity.
        #[fail(display = "The targetted zone is full")]
        ZoneFull,
    }

    /// Code failed to push a new item onto the chosen stack.
    #[derive(Debug, Fail)]
    #[fail(display = "Error unpacking the provided transaction")]
    pub struct TransactionUnpackError;

    /// Code failed to push a new item onto the chosen stack.
    #[derive(Debug, Fail)]
    #[fail(display = "Error pushing data to the stack")]
    pub struct StackPushError;

    /// Code failed to push a new item onto the chosen stack.
    #[derive(Debug, Fail)]
    #[fail(display = "Error popping data from the stack")]
    pub struct StackPopError;

    /// Specific error thrown to indicate the system cannot execute the request under
    /// constrained circumstances.
    #[derive(Debug, Fail)]
    #[fail(display = "A constraint amount is overflowed, maximum is {:}", _0)]
    pub struct OverflowError(pub usize);

    /// Code failed to get a mutable reference to an [`Entity`].
    #[derive(Debug, Fail)]
    #[fail(display = "The entity cannot be unwrapped mutably")]
    pub struct InvalidEntityMutUnwrap;

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

    impl<ID> Fail for MissingEntityError<ID>
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

    /// Specific error thrown when the requested card-id is not known.
    #[derive(Debug)]
    pub struct MissingCardError<ID>(pub ID)
    where
        ID: Display + Debug;

    impl<ID> Fail for MissingCardError<ID>
    where
        ID: Display + Debug + Send + Sync + 'static,
    {
    }

    impl<ID> fmt::Display for MissingCardError<ID>
    where
        ID: Display + Debug,
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "The card with id `{:}` was not found", self.0)
        }
    }

    /// Specific error thrown when the requested property is not known.
    #[derive(Debug)]
    pub struct MissingPropertyError<ID, PROP>(pub ID, pub PROP)
    where
        ID: Display + Debug,
        PROP: Debug;

    impl<ID, PROP> Fail for MissingPropertyError<ID, PROP>
    where
        ID: Display + Debug + Send + Sync + 'static,
        PROP: Debug + Send + Sync + 'static,
    {
    }

    impl<ID, PROP> fmt::Display for MissingPropertyError<ID, PROP>
    where
        ID: Display + Debug,
        PROP: Debug,
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "The property `{:?}` was not found for entity `{:}`",
                self.1, self.0
            )
        }
    }

    /// Specific error thrown when the requested entity-id is not known.
    #[derive(Debug)]
    pub struct MissingPrototypeError<ID, P>(pub ID, pub P)
    where
        ID: Display + Debug,
        P: marker::ProtoEnumerator + Debug;

    impl<ID, P> Fail for MissingPrototypeError<ID, P>
    where
        ID: Display + Debug + Send + Sync + 'static,
        P: marker::ProtoEnumerator + Debug + Send + Sync + 'static,
    {
    }

    impl<ID, P> fmt::Display for MissingPrototypeError<ID, P>
    where
        ID: Display + Debug,
        P: marker::ProtoEnumerator + Debug,
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "The entity with id `{:}` doesn't have the prototype `{:?}`",
                self.0, self.1
            )
        }
    }

    /// Error thrown when the provided object's ID collides with an already known ID.
    #[derive(Debug)]
    pub struct IDCollisionError<ID>(pub ID)
    where
        ID: Display + Debug;

    impl<ID> Fail for IDCollisionError<ID>
    where
        ID: Display + Debug + Send + Sync + 'static,
    {
    }

    impl<ID> fmt::Display for IDCollisionError<ID>
    where
        ID: Display + Debug,
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "An object with ID {:} is already registered", self.0)
        }
    }

    /// Enumeration of failure cases working with [`Trigger`]s.
    #[derive(Debug, Fail, Copy, Clone, Eq, PartialEq)]
    pub enum TriggerFail {
        /// Error indicating the callback pointer is invalid.
        #[fail(display = "The provided callback pointer is invalid")]
        CallbackNull,
        /// Error indicating the provided machine does not validate the [`Trigger`]
        /// constraints.
        #[fail(display = "The provided machine does not validate on the constraints")]
        ConstraintFail,
    }
}
