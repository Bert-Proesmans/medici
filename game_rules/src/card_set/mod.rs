//! Contains all game card definitions.

pub mod core;
pub mod test;

/// Enumeration of all known sets of cards.
pub enum CardSet {
    /// Core card set, see module [`core`].
    Core = 0,
    /// Test card set, see module [`test`].
    Test = 1,
}

// Necessary for usage with CardId.
impl From<CardSet> for u32 {
    fn from(x: CardSet) -> u32 {
        x as u32
    }
}
