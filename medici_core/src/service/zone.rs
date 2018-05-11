//! Module containing zone related structs.

use std::hash::Hash;

use ctmut::{CTBool, MutSwitch, CTTrue, CTFalse};
use function::{Entity, EntityId, Identifiable, ZoneEnumerator, AdapterCompliant};
use marker;
use storage::ZoneStorage;
use service::EntityService;

#[derive(Debug)]
/// Structure that allows for manipulating zone data. This behaviour
/// is provided by abstracting over the entity storage structure.
// Note: Only one lifetime parameter is used because we are only interested
// in exactly one constraint: Fields (and containing references) outlive
// the adapter struct (lifetime a). The adapter does NOT outlive 'a, so 
// all containing references are valid up to 'a. The constraint itself is
// formulated as [lifetime of entities]: 'a, the [lifetime of entities]
// is at least as long as the lifetime a.
pub struct ZoneAdapter<'a, AllowMut, E, ZE>
where
    AllowMut: CTBool,
    E: Entity + Identifiable<ID = EntityId> + Clone,
    ZE: ZoneEnumerator + Hash + Eq + Default,
    EntityService<E>: 'a,
    ZoneStorage<E, ZE>: 'a,
{
    entities: MutSwitch<'a, EntityService<E>, AllowMut>,
    zones: MutSwitch<'a, ZoneStorage<E, ZE>, AllowMut>,
}

impl<'a, AllowMut, E, ZE> marker::Service for ZoneAdapter<'a, AllowMut, E, ZE>
where
    AllowMut: CTBool,
    E: Entity + Identifiable<ID = EntityId> + Clone,
    ZE: ZoneEnumerator + Hash + Eq + Default,
{
}

impl<'a, AllowMut, E, ZE> marker::Adapter for ZoneAdapter<'a, AllowMut, E, ZE>
where
    AllowMut: CTBool,
    E: Entity + Identifiable<ID = EntityId> + Clone,
    ZE: ZoneEnumerator + Hash + Eq + Default,
{
    type Adapting = EntityService<E>;
}

#[derive(Debug)]
/// Structure for creating an [`Adapter`] that allows manipulation of zone information.
/// 
/// Do not use this stub directly! The purpose of this stub is to own the zone information.
/// Create an [`Adapter`] from this type and [`EntityService`] to manipulate zone information!
pub struct ZoneServiceStub<E, ZE> 
where
    E: Entity + Identifiable<ID = EntityId> + Clone,
    ZE: ZoneEnumerator + Hash + Eq + Default,
{
    storage: ZoneStorage<E, ZE>,
}

impl<E, ZE> ZoneServiceStub<E, ZE> 
where
    E: Entity + Identifiable<ID = EntityId> + Clone,
    ZE: ZoneEnumerator + Hash + Eq + Default,
{
    /// Constructs a new service stub which owns the zone information data.
    pub fn new() -> Self {
        Self {
            storage: ZoneStorage::new(),
        }
    }
}

impl<'a, E, ZE> AdapterCompliant<'a, ZoneAdapter<'a, CTFalse, E, ZE>> for ZoneServiceStub<E, ZE> 
where
    E: Entity + Identifiable<ID = EntityId> + Clone,
    ZE: ZoneEnumerator + Hash + Eq + Default,
{
    fn build(&'a self, service: &'a EntityService<E>) -> ZoneAdapter<CTFalse, E, ZE> {
        ZoneAdapter {
            entities: MutSwitch::from_ref(service),
            zones: MutSwitch::from_ref(&self.storage),
        }
    }
}
