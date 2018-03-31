//! Contains the functionality to work with triggers.

use std::cmp::PartialEq;
use std::marker::PhantomData;
use std::convert::TryFrom;

use failure::Error;
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

    /// Consumes this wrapper to retrieve the callback it contains.
    pub fn into_callback(self) -> _FNTrigger<M> {
        self.cb
    }
}

impl<M, ETM, ETR> TryFrom<TriggerEntry<ETM, ETR>> for TriggerWrapper<M, ETM, ETR>
where
    M: StateContainer,
    M::State: TriggerState,
    <M::State as TriggerState>::Timing: Timing + IntoEnum<ETM>,
    <M::State as TriggerState>::Trigger: Trigger + IntoEnum<ETR>,
    ETM: TimingEnumerator + PartialEq + Copy,
    ETR: TriggerEnumerator + PartialEq + Copy,
{
    type Error = Error;

    fn try_from(x: TriggerEntry<ETM, ETR>) -> Result<Self, Self::Error> {
        let timing_key: ETM = <M::State as TriggerState>::Timing::into_enum();
        let trigger_key: ETR = <M::State as TriggerState>::Trigger::into_enum();

        if x.cb.is_null() {
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
            let transmuted: _FNTrigger<M> = ::std::mem::transmute(x.cb);
            Ok(Self {
                cb: transmuted,
                phantom: PhantomData,
            })
        }
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
            cb: x.cb as *const (),
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
/// Note: This could be enforced by letting the StateContainer return a unique
/// number for each created object. This number could be used as constraint
/// when reconstructing [`TriggerWrapper`].
#[derive(Debug)]
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
