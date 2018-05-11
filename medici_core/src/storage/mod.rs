//! Types with each different functionality to store data.

pub mod card;
pub mod entity;
pub mod transaction;
pub mod trigger;
pub mod zone;

pub use self::card::CardStorage;
pub use self::entity::EntityStorage;
pub use self::transaction::TransactionStorage;
pub use self::trigger::TriggerStorage;
pub use self::zone::ZoneStorage;
