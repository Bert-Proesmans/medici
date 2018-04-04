//! Contains the functionality to work with triggers.

use std::cmp::PartialEq;
use std::marker::PhantomData;

use failure::{format_err, Error};
use value_from_type_traits::IntoEnum;

use function::{StateContainer, TriggerState};
use marker::{Service, Timing, TimingEnumerator, Trigger, TriggerEnumerator};
use service::storage::{TriggerEntry, TriggerStorage};

// Shortcut for a callback method prototype which consumes the machine
// and returns it again.. or a generic error.
//
// TODO; Transfrom Error into a real error type.
type _FNTrigger<M> = fn(M) -> Result<M, Error>;

/// Safe abstraction over TriggerEntry objects.
#[derive(Debug)]
pub struct TriggerWrapper<M, ETM, ETR>
where
    M: StateContainer,
    M::State: TriggerState,
    <M::State as TriggerState>::Timing: Timing + IntoEnum<ETM>,
    <M::State as TriggerState>::Trigger: Trigger + IntoEnum<ETR>,
    ETM: TimingEnumerator + PartialEq + Copy,
    ETR: TriggerEnumerator + PartialEq + Copy,
    // Additional constraint inference -> *const (): Send + Sync
    _FNTrigger<M>: Send + Sync,
{
    cb: _FNTrigger<M>,
    phantom: PhantomData<(ETM, ETR)>,
}

impl<M, ETM, ETR> TriggerWrapper<M, ETM, ETR>
where
    M: StateContainer,
    M::State: TriggerState,
    <M::State as TriggerState>::Timing: Timing + IntoEnum<ETM>,
    <M::State as TriggerState>::Trigger: Trigger + IntoEnum<ETR>,
    ETM: TimingEnumerator + PartialEq + Copy,
    ETR: TriggerEnumerator + PartialEq + Copy,
{
    fn new(cb: _FNTrigger<M>) -> Self {
        Self {
            cb,
            phantom: PhantomData,
        }
    }

    /// Build a safe wrapper from a [`TriggerEntry`] object.
    ///
    /// # Safety
    /// The exact [`StateContainer`] (state machine) is removed from the [`TriggerEntry`].
    /// A safe wrapper can thus be generated for ANY state machine which [`TriggerState`]-associated
    /// types Timing and Trigger match on the requested machine.
    ///
    /// See [`TriggerService`] for more information!
    pub unsafe fn try_from_trigger_entry(x: TriggerEntry<ETM, ETR>) -> Result<Self, Error> {
        let timing_key: ETM = <M::State as TriggerState>::Timing::into_enum();
        let trigger_key: ETR = <M::State as TriggerState>::Trigger::into_enum();

        if x.func_pointer.is_null() {
            // TODO: Transform into real error!
            return Err(format_err!("Callback is NULL!"));
        }

        if x.timing != timing_key || x.trigger != trigger_key {
            // TODO; Transform into real error!
            return Err(format_err!("Incompatible layout!"));
        }

        // Proceed with converting the callback pointer into an FN type
        // compatible with the current Timing and Trigger.
        //
        // Note(UB):
        // This uses the provided State-Machine [M] as conversion template
        // but there is nothing enforcing usage of the CORRECT machine!
        unsafe {
            let transmuted: _FNTrigger<M> = ::std::mem::transmute(x.func_pointer);
            Ok(Self {
                cb: transmuted,
                phantom: PhantomData,
            })
        }
    }

    /// Consumes this wrapper to retrieve the callback it contains.
    pub fn into_callback(self) -> _FNTrigger<M> {
        self.cb
    }
}

impl<M, ETM, ETR> From<TriggerWrapper<M, ETM, ETR>> for TriggerEntry<ETM, ETR>
where
    M: StateContainer,
    M::State: TriggerState,
    <M::State as TriggerState>::Timing: Timing + IntoEnum<ETM>,
    <M::State as TriggerState>::Trigger: Trigger + IntoEnum<ETR>,
    ETM: TimingEnumerator + PartialEq + Copy,
    ETR: TriggerEnumerator + PartialEq + Copy,
{
    fn from(x: TriggerWrapper<M, ETM, ETR>) -> Self {
        let timing_key: ETM = <M::State as TriggerState>::Timing::into_enum();
        let trigger_key: ETR = <M::State as TriggerState>::Trigger::into_enum();

        Self {
            timing: timing_key,
            trigger: trigger_key,
            // Transformation is performed here to make the handler generic
            // for storage.
            func_pointer: x.cb as *const (),
            //
            _private: PhantomData,
        }
    }
}

/// Structure wrapping and containing all [`Trigger`]s registered on the
/// state machine.
///
/// # Safety
/// This service erases the exact state-machine type by only keeping Timing
/// and Trigger information.
/// DO NOT execute callbacks built from a specific machine with another machine!
///
/// TODO: Think about enforcing this check at compile time.
///
/// Note: This could be enforced by letting the StateContainer return a unique
/// number for each created object. This number could be used as constraint
/// when reconstructing [`TriggerWrapper`].
///
/// Note: The trigger methods could be written as if they respond to a certain [`Timing`]
/// and [`Trigger`] where the exact machine format is abstracted.
/// Additionally, machine constraints can be defined on these methods to statically
/// enforce services.
/// This requires a lot of boilerplate code but macro's might be able to solve this?
///
/// eg:
/// ```rust
///
/// #struct Post;
/// #impl Timing for Post {}
///
/// #struct EndTurn;
/// #impl Trigger for EndTurn {}
///
/// fn print_trigger<M>(machine: M) -> Result<M, Error>
/// where
///     M: StateContainer,
///     M::State: TriggerState<Timing=Post, Trigger=EndTurn>,
/// {
///     unimplemented!()
/// }
/// ```
#[derive(Debug, Clone)]
pub struct TriggerService<ETM, ETR>
where
    ETM: TimingEnumerator + PartialEq + Copy,
    ETR: TriggerEnumerator + PartialEq + Copy,
{
    storage: TriggerStorage<ETM, ETR>,
}

impl<ETM, ETR> Service for TriggerService<ETM, ETR>
where
    ETM: TimingEnumerator + PartialEq + Copy,
    ETR: TriggerEnumerator + PartialEq + Copy,
{
}

impl<ETM, ETR> TriggerService<ETM, ETR>
where
    ETM: TimingEnumerator + PartialEq + Copy,
    ETR: TriggerEnumerator + PartialEq + Copy,
{
    /// Creates a new object of this service.
    pub fn new() -> Self {
        Self {
            storage: TriggerStorage::new(),
        }
    }

    /// Add a new trigger to the store.
    pub fn add_trigger<M>(&mut self, cb: _FNTrigger<M>)
    where
        M: StateContainer,
        M::State: TriggerState,
        <M::State as TriggerState>::Timing: Timing + IntoEnum<ETM>,
        <M::State as TriggerState>::Trigger: Trigger + IntoEnum<ETR>,
    {
        // Both the new method AND the Into trait will do the hard work for us!
        let safe_wrapper = TriggerWrapper::<M, ETM, ETR>::new(cb);
        // TODO; Abstract the triggers field of TriggerStorage!
        self.storage.triggers.push(safe_wrapper.into());
    }

    /// Retrieve all triggers matching the provided machine.
    ///
    /// # Borrow-check
    /// This method might cause issues with the borrow checker because [`Service`] is designed
    /// to be contained by a state machine. By accessing this specific service we place
    /// an immutable borrow onto that machine, which is also passed as parameter into
    /// this method.
    /// In general this additional immutable borrow should not matter.
    /// Returning [`TriggerEntry`] references will limit accessibility into the machine and
    /// this service.
    ///
    /// The latter practically forces framework users to instantly make a copy of each returned
    /// [`TriggerEntry`] reference. Ultimately we leave the choice of usage up to the framework
    /// user. The reason being that we want additional functional operations to be as lightweight
    /// as possible.
    pub fn retrieve_triggers<M>(&self, m: &M) -> impl Iterator<Item = &TriggerEntry<ETM, ETR>>
    where
        M: StateContainer,
        M::State: TriggerState,
        <M::State as TriggerState>::Timing: Timing + IntoEnum<ETM>,
        <M::State as TriggerState>::Trigger: Trigger + IntoEnum<ETR>,
    {
        let timing_key: ETM = <M::State as TriggerState>::Timing::into_enum();
        let trigger_key: ETR = <M::State as TriggerState>::Trigger::into_enum();

        self.storage
            .triggers
            .iter()
            .filter(move |e| e.timing == timing_key)
            .filter(move |e| e.trigger == trigger_key)
        // Note: We could map a safe wrapper on top of this iterator?
    }
}