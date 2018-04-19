use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use failure::Error;
use indexmap::set::IndexSet;
use maplit::hashmap;

use function::{Entity, ZoneEnumerator};

#[derive(Debug, Clone)]
/// Structure for assigning zones to entities.
pub struct ZoneEntityStorage<E, EZ>
where
    E: Entity,
    <E as Entity>::ID: Copy + Debug + PartialEq + Eq + Hash,
    EZ: ZoneEnumerator + Default + Clone + Debug + PartialEq + Eq + Hash,
{
    zone_assignment: HashMap<EZ, IndexSet<E::ID>>,
}

impl<E, EZ> ZoneEntityStorage<E, EZ>
where
    E: Entity,
    <E as Entity>::ID: Copy + Debug + PartialEq + Eq + Hash,
    EZ: ZoneEnumerator + Default + Clone + Debug + PartialEq + Eq + Hash,
{
    /// Creates a new storage object which keeps track of entities located/moving
    /// between zones.
    pub fn new() -> Self {
        Self {
            zone_assignment: hashmap!{},
        }
    }

    /*
     * Note: Zone operations require both the container AND the entity to be mutable!
     * Having both mutable means we can't introduce unintuitive results because of the
     * loose coupling between zones and entities.
     */

    /// Event for a newly created entity.
    ///
    /// This method will insert a new zone-entry for the provided [`Entity`].
    pub(crate) fn new_entity(&mut self, _entity: &mut E) -> Result<(), Error> {
        unimplemented!()
    }

    /// Moves the provided entity into the given zone.
    pub fn move_zone(&mut self, _entity: &mut E, _zone: EZ) -> Result<(), Error> {
        unimplemented!()
    }

    /// Moves the provided entity to the provided position within it's current zone.
    ///
    /// TODO; Figure out how to quickly retrieve this zone, either;
    ///     * Implement hidden properties for [`Entity`] and use them only within core.
    ///     * Implement a two-way mapping between entities and zones.
    pub fn move_position(&mut self, _entity: &mut E, _position: usize) -> Result<(), Error> {
        unimplemented!()
    }
}
