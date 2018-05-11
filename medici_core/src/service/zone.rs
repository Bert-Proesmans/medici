//! Module containing zone related structs.

use std::hash::Hash;

use ctmut::{CTBool, MutSwitch};
use function::{Entity, EntityId, Identifiable, ZoneEnumerator};
use marker;
use storage::{EntityStorage, ZoneStorage};

/// Structure that allows for manipulating zone data. This behaviour
/// is provided by abstracting over the entity storage structure.
pub struct ZoneAdapter<'a, 'b, AllowMut, E, ZE>
where
    AllowMut: CTBool,
    E: Entity + Identifiable<ID = EntityId> + Clone,
    ZE: ZoneEnumerator + Hash + Eq + Default,
    EntityStorage<E>: 'a,
    ZoneStorage<E, ZE>: 'b,
{
    entities: MutSwitch<'a, EntityStorage<E>, AllowMut>,
    zones: MutSwitch<'b, ZoneStorage<E, ZE>, AllowMut>,
}

impl<'a, 'b, AllowMut, E, ZE> marker::Service for ZoneAdapter<'a, 'b, AllowMut, E, ZE>
where
    AllowMut: CTBool,
    E: Entity + Identifiable<ID = EntityId> + Clone,
    ZE: ZoneEnumerator + Hash + Eq + Default,
{
}

impl<'a, 'b, AllowMut, E, ZE> marker::Adapter for ZoneAdapter<'a, 'b, AllowMut, E, ZE>
where
    AllowMut: CTBool,
    E: Entity + Identifiable<ID = EntityId> + Clone,
    ZE: ZoneEnumerator + Hash + Eq + Default,
{
    type Adapting = EntityStorage<E>;
}

// struct ZoneService {
// 	storage: ZoneStorage,
// }
