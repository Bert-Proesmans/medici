//! Contains the core functionality items for our system.
use std::fmt;

use ctstack::CTStack;
use error::custom_type::StackPopError;
use marker;

/// Trait generalizing over any structure that could act as a container of states.
///
/// This container of states could be reworded as 'the state machine' itself.
pub trait StateContainer {
    /// Type of the current state held by the state machine.
    type State: State;
    /// Type of transaction object necessary to transition into the
    /// current state of the machine.
    type Transaction: marker::Transaction = <Self::State as State>::Transaction;
    /// Type which enumerates all possible timings contained by the machine.
    type TimingEnum: marker::TimingEnumerator;
    /// Type which enumerates all possible triggers contained by the machine.
    type TriggerEnum: marker::TriggerEnumerator;
    /// Type representing the stack of types where the container state was
    /// transitioned in a pushdown manner.
    type TransitionRecord: CTStack;
}

/// Trait generalizing over any state that's present in the state machine.
pub trait State {
    /// Type of structure which must be provided when transitioning into the state
    /// represented by the enclosing type.
    type Transaction: marker::Transaction;
}

/// Trait generalizing over any state which is used to bootstrap an execution of triggers.
pub trait EffectState: State {}

/// Trait generalizing over any state that's used to pass into trigger callbacks
/// when trigger conditions are met.
pub trait TriggerState: State {
    /// Encoded type value representing the timing (related to triggers) of the
    /// current state.
    type Timing: marker::Timing;
    /// Encoded type value representing the trigger of the current state.
    type Trigger: marker::Triggerable;
}

/// Trait enforcing implementing objects to expose their identifier.
/// The uniqueness of this identifier depends on the implementing object itself.
pub trait Identifiable {
    /// The type of identifier used to pass between functions during state machine
    /// execution.
    /// This type MUST be [`Copy`] because storing this identifier is the idiomatic
    /// way of passing "references" around.
    type ID: Copy;

    /// Returns the identifier of the implementing object.
    fn id(&self) -> Self::ID;
}

/// Type that's generally used to identify and order [`Entity`] objects.
///
/// Throughout medici-core it's assumed this type is an alias for a numeric
/// type!
pub type EntityId = usize;

/// Trait representing an object which properties can be altered dynamically (at runtime).
///
/// # Identifiable
/// Entity objects receive an identifier which is only valid for the machine that created
/// it.
///
/// # Note
/// This trait MUST ALWAYS be object safe!
/// This provides the flexibility to store a bunch of [`Entity`]s into one container.
pub trait Entity: Identifiable {
    //
}

/// Trait used to create a new [`Entity`] object.
pub trait EntityBuilder<E: Entity> {
    /// Build a new [`Entity`] with the provided identifier.
    fn new_with_id(id: E::ID) -> E;
}

/// Trait representing an actual game card.
///
/// A card is an [`Entity`] but it's usage is semantically disjunct enough to warrant
/// a seperate type.
///
/// # Identifiable
/// Cards are identified with GLOBAL UNIQUE identifiers.
/// This allows for cards to be constructed once and a static table could be generated
/// to retrieve a reference from there.
/// It's also possible to hand out owned cards given their identifier doesn't clash with
/// any other card.
///
/// # Note
/// This trait MUST ALWAYS be object safe!
/// This provides the flexibility to store a bunch of [`Card`]s into one container.
pub trait Card: Identifiable {
    /// All timing types this card holds listeners for.
    type TimingEnum: marker::TimingEnumerator;
    /// All trigger types this card holds listeners for.
    type TriggerEnum: marker::TriggerEnumerator;
}

/// Trait used to create a new [`Card`] object.
pub trait CardBuilder<C: Card> {
    /// Build a new [`Card`] with the provided identifier.
    fn new_with_id<I: Into<C::ID>>(id: I) -> C;
}

/// Types that construct an [`Adapter`] around some [`Service`].
///
/// The adapter is built from service stubs, which own additional data, and the
/// selected service. The adapter often contains nothing more than borrows of
/// services and/or storage objects.
///
/// # See also
/// [`marker::Adapter`]
///
// Note: This trait holds an explicit lifetime because all borrows that go into the
// adapter must outlive that adapter (A: 'a).
// &'a self == self outlives lifetime a.  OR
// the reference to self is valid up to lifetime a.
// When no lifetime is specified (lifetime ellision), the compiler will insert a new
// one for us automatically. For example lifetime "unknown".
// 'unknown is always strictly shorter than 'a because of scoping. The compiler cannot
// constraint make 'unknown == 'a if no constraints are provided
// ('unknown: 'a == lifetime unknown lives at least as long as lifetime a)
pub trait AdapterCompliant<'a, A>
where
    A: marker::Adapter + 'a,
{
    /// Creates an adapter around the provided service.
    fn build(&'a self, service: &'a A::Adapting) -> A;
}

/// Types that construct an [`Adapter`] around some [`Service`].
///
/// # See also
/// [`AdapterCompliant`]
pub trait AdapterCompliantMut<'a, A>
where
    A: marker::Adapter + 'a,
{
    /// Creates an adapter around the provided service.
    fn build_mut(&'a mut self, service: &'a mut A::Adapting) -> A;
}

/// Trait for implementing a certain service on the state machine.
///
/// Because of this design exactly one object of each service type can be hooked onto
/// the same state machine.
pub trait ServiceCompliance<S>
where
    S: marker::Service,
{
    /// Retrieves an immutable reference to service `S`.
    fn get(&self) -> &S;
    /// Retrieves a mutable reference to service `S`.
    fn get_mut(&mut self) -> &mut S;
}

/// Defines stack behaviour for a certain storage object.
pub trait StackStorageCompliance {
    /// The type of items found within the implementing storage.
    type Item;

    /// Adds the provided item onto this stack.
    fn push<I: Into<Self::Item>>(&mut self, _: I);

    /// Removes the top most item of the stack.
    ///
    /// The top most item is the one which was pushed last before
    /// executing this method.
    fn pop(&mut self) -> Result<Self::Item, StackPopError>;
}

/// Defines indexed behaviour for a certain storage object.
pub trait IndexedStorageCompliance {
    /// The type of items found within the implementing storage.
    type Item: Identifiable;

    /// Retrieves a reference to the requested item matching the provided
    /// identifier.
    fn get(&self, identifier: <Self::Item as Identifiable>::ID) -> Option<&Self::Item>;

    /// Retrieves a reference to the requested item matching the provided
    /// identifier.
    fn get_mut(&mut self, identifier: <Self::Item as Identifiable>::ID) -> Option<&mut Self::Item>;
}

/// Defines array access behaviour for storage objects.
pub trait ArrayStorageCompliance {
    /// The type of items found within the implementing storage.
    type Item;

    /// Returns the current storage as a slice, which is an indexed storage.
    fn as_slice(&self) -> &[Self::Item];

    /// Returns the current storage as a slice, which is an indexed storage.
    fn as_slice_mut(&mut self) -> &mut [Self::Item];
}

/// Types which enumerate all entity zones in the game.
pub trait ZoneEnumerator {
    /// Returns the amount of entities this zone can hold.
    fn max_entities(&self) -> usize;
}

/* ID sructures */

#[derive(Debug, Clone, Copy)]
/// Type that's generally used to identify and order [`Card`] objects.
///
/// The first numeric element is the SET IDENTIFIER.
/// The second numeric element is the ordinal identifier within the set.
///
pub struct CardId(u32, u32);
impl fmt::Display for CardId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<Card(SET={},ID={})>", self.0, self.1)
    }
}

impl CardId {
    /// Creates a new identifier structure for a card.
    pub const fn new(set: u32, id: u32) -> Self {
        CardId(set, id)
    }

    /// Creates a new identifier structure for a card.
    pub fn from_set<S: Into<u32>>(set: S, id: u32) -> Self {
        CardId(set.into(), id)
    }
}
