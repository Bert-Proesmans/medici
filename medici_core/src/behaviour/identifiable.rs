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
pub trait EntityBuilder {
    type Output: Entity;

    /// Build a new [`Entity`] with the provided identifier.
    fn new_with_id<I>(id: I) -> Self::Output
    where
        I: Into<<Self::Output as Identifiable>::ID>;
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
    //
}

/// Trait used to create a new [`Card`] object.
pub trait CardBuilder {
    type Output: Card;

    /// Build a new [`Card`] with the provided identifier.
    fn new_with_id<I>(id: I) -> Self::Output
    where
        I: Into<<Self::Output as Identifiable>::ID>;
}
