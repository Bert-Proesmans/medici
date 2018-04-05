use std::marker::PhantomData;

use marker::{TimingEnumerator, TriggerEnumerator};

/// Structure serializng/generalizing a trigger.
///
/// The state machine will use this abstraction to store a specific
/// method which has to be executed when the trigger conditions are valid.
/// This type also makes triggers portable so they can be freely moved
/// and copied through parts of the machine.
#[derive(Debug, Clone)]
pub struct UnsafeTrigger<ETM, ETR>
where
    ETM: TimingEnumerator + Copy,
    ETR: TriggerEnumerator + Copy,
{
    /// (Sized) Timing value belonging to the callback, see func_pointer.
    pub timing: ETM,
    /// (Sized) Trigger value belonging to the callback, see func_pointer.
    pub trigger: ETR,
    /// The callback pointer which must be transmuted and executed
    /// when the conditions of the running state machine match the
    /// ones contained within this structure.
    pub func_pointer: *const (),

    // This field prevents UnsafeTrigger from being constructed by framework
    // users.
    pub(crate) _private: PhantomData<()>,
}

unsafe impl<ETM, ETR> Send for UnsafeTrigger<ETM, ETR>
where
    ETM: TimingEnumerator + Copy + Send,
    ETR: TriggerEnumerator + Copy + Send,
{
}

unsafe impl<ETM, ETR> Sync for UnsafeTrigger<ETM, ETR>
where
    ETM: TimingEnumerator + Copy + Sync,
    ETR: TriggerEnumerator + Copy + Sync,
{
}

/// Structure used to store a portable format of trigger entries, see [`UnsafeTrigger`].
#[derive(Debug, Clone)]
pub struct TriggerStorage<ETM, ETR>
where
    ETM: TimingEnumerator + Copy,
    ETR: TriggerEnumerator + Copy,
{
    // pub pre_action_triggers: Vec<UnsafeTrigger<ETM, ETR>>,
    // pub peri_action_triggers: Vec<UnsafeTrigger<ETM, ETR>>,
    // pub post_action_triggers: Vec<UnsafeTrigger<ETM, ETR>>,
    /// Container of all triggers registered with the running state machine.
    ///
    /// TODO; Find out if splitting this up is useful + find out how.
    pub triggers: Vec<UnsafeTrigger<ETM, ETR>>,
}

impl<ETM, ETR> TriggerStorage<ETM, ETR>
where
    ETM: TimingEnumerator + Copy,
    ETR: TriggerEnumerator + Copy,
{
    /// Builds a new object for storage.
    pub fn new() -> Self {
        Self { triggers: vec![] }
    }
}
