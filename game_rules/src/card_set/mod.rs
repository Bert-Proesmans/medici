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

// // Make sure any variation of Card ID will be accepted by Cards and
// // Card Storage objects.
// use game_system::re_export::function::CardId;

// impl From<(CardSet, u16)> for CardId {
//     fn from(x: (CardSet, u16)) -> CardId {
//     	(x.0 as u16, x.1)
//     }
// }
