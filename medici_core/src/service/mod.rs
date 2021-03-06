//! Types that provide functionality to your application.
//! These types can be added AD-HOC to your (state) machine
//! declaration.
//! Other components of this framework also need some of these
//! services to be implemented before use.

pub mod card;
pub mod entity;
pub mod trigger;

pub use self::card::CardService;
pub use self::entity::EntityService;
pub use self::trigger::TriggerService;
