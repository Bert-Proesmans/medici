//! Module containing structures for storing zone information.

use std::collections::HashMap;
use std::hash::Hash;

use error::custom_type::ZoneMoveError;
use function::{Entity, ZoneEnumerator};

#[derive(Debug)]
/// Structure containing the zone assignment for each [`Entity`].
pub struct ZoneStorage<E, ZE>
where
    E: Entity,
    ZE: ZoneEnumerator + Hash + Eq + Default,
{
    /// Hashmap containing a sorted list of entities for each zone.
    /// The position of each entity within that list represents the position
    /// within that zone.
    pub zone_assignment: HashMap<ZE, Vec<E::ID>>,
}

impl<E, ZE> ZoneStorage<E, ZE>
where
    E: Entity,
    ZE: ZoneEnumerator + Hash + Eq + Default,
{
    /// Creates a new storage object which keeps track of entities
    /// and their zone assignment.
    pub fn new() -> Self {
        Self {
            zone_assignment: hashmap!{},
        }
    }

    /// Moves the provided [`Entity`] from its current zone into the targetted zone.
    pub fn zone_move(&mut self, entity: &mut E, from: ZE, to: ZE) -> Result<(), ZoneMoveError> {
        unimplemented!()
    }
}
