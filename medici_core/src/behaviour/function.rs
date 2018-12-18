use crate::behaviour::marker;
use crate::compile_tools as ct;

/// Supertrait of types that contain all information for keeping a game state machine
/// running.
///
/// The most important behaviours are defined here and you're encouraged to extend this trait
/// with your own behaviour.
///
/// Using a Trait instead of an explcit type allows handlers and other interaction code to work
/// on specific properties of the machine, while remaining generic for non-relevant properties.
pub trait StateMachine {
    /// Type of the current state held by the state machine.
    type State: State;
    /// Type which enumerates all possible timings contained by the machine.
    type TimingEnum: marker::TimingComparator;
    /// Type which enumerates all possible triggers contained by the machine.
    type TriggerEnum: marker::TriggerComparator;
    /// Type representing the stack of types where the container state was
    /// transitioned in a pushdown manner.
    type TransitionRecord: ct::Stack;
}

/// Supertrait of all types that represent a specific state of the game.
/// States are used within the State Machine to group and execute game logic.
pub trait State {
    /// Type of structure value that must be provided when transitioning into the current
    /// state.
    type Transaction: marker::Transaction;
}

/// Supertrait of machine states that bootstrap execution of a chain of triggers. These triggers
/// are bound to the concrete effect AND could recurse!
pub trait EffectState: State {}

/// Supertrait of machine states that handle execution of a set of triggers. This set of triggers
/// is created by filtering stored handlers with the concrete trigger conditions.
pub trait TriggerState: State {
    /// Encoded type value representing the timing (related to triggers) of the
    /// current state.
    type Timing: Timing;
    /// Encoded type value representing the trigger of the current state.
    type Trigger: Trigger;
}

/// Supertrait of state types that encode timing.
pub trait Timing: Copy + Sized {
    type ComparatorType: marker::TimingComparator;

    const COMPARATOR: Self::ComparatorType;
}

/// Supertrait of state types that encode an event. Handlers can be registered for this event.
pub trait Trigger: Copy + Sized {
    type ComparatorType: marker::TriggerComparator;

    const COMPARATOR: Self::ComparatorType;
}
