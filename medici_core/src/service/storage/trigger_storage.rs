use marker::{TimingEnumerator, TriggerEnumerator};

/// Structure serializng/generalizing a trigger.
///
/// The state machine will use this abstraction to store a specific
/// method which has to be executed when the trigger conditions are valid.
/// This type also makes triggers portable so they can be freely moved
/// and copied through parts of the machine.
#[derive(Debug)]
pub struct TriggerEntry<ETM, ETR>
where
    ETM: TimingEnumerator + Copy,
    ETR: TriggerEnumerator + Copy,
{
    /// (Sized) Timing value belonging to the callback, see cb.
    pub timing: ETM,
    /// (Sized) Trigger value belonging to the callback, see cb.
    pub trigger: ETR,
    /// The callback pointer which must be transmuted and executed
    /// when the conditions of the running state machine match the
    /// ones contained within this structure.
    pub cb: *const (),
}

/// Structure used to store a portable format of trigger entries, see [`TriggerEntry`].
#[derive(Debug)]
pub struct TriggerStorage<ETM, ETR>
where
    ETM: TimingEnumerator + Copy,
    ETR: TriggerEnumerator + Copy,
{
    // pub pre_action_triggers: Vec<TriggerEntry<ETM, ETR>>,
    // pub peri_action_triggers: Vec<TriggerEntry<ETM, ETR>>,
    // pub post_action_triggers: Vec<TriggerEntry<ETM, ETR>>,
    /// Container of all triggers registered with the running state machine.
    ///
    /// TODO; Find out if splitting this up is useful + find out how.
    pub triggers: Vec<TriggerEntry<ETM, ETR>>,
}
