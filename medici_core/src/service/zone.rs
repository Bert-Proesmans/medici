//! Module containing zone related structs.

use std::hash::Hash;

use ctmut::{CTBool, CTFalse, CTTrue, MutSwitch};
use function::{
    AdapterCompliant, AdapterCompliantMut, Entity, EntityBuilder, EntityId, Identifiable,
    ZoneEnumerator,
};
use marker;
use service::EntityService;
use storage::ZoneStorage;

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
    entity_wrapper: MutSwitch<'a, EntityService<E>, AllowMut>,
    zone_wrapper: MutSwitch<'a, ZoneStorage<E, ZE>, AllowMut>,
}

impl<'a, E, ZE> ZoneAdapter<'a, CTFalse, E, ZE>
where
    E: Entity + Identifiable<ID = EntityId> + EntityBuilder<E> + Clone,
    EntityId: Clone,
    ZE: ZoneEnumerator + Hash + Eq + Default,
{
    /// Returns an iterator over all entities within the provided zone.
    pub fn iter_zone(&self, zone: ZE) -> impl Iterator<Item = &E> {
        let entity_service = self.entity_wrapper.get();
        let zone_storage = self.zone_wrapper.get();
        zone_storage
            .zone_assignment
            .get(&zone)
            .map_or_else(|| [].iter(), |z| z.iter())
            .map(move |e_id| entity_service.get(*e_id).unwrap())
    }
}

impl<'a, E, ZE> ZoneAdapter<'a, CTTrue, E, ZE>
where
    E: Entity + Identifiable<ID = EntityId> + EntityBuilder<E> + Clone,
    EntityId: Clone,
    ZE: ZoneEnumerator + Hash + Eq + Default,
{
    /// Returns an iterator over all entities within the provided zone.
    pub fn iter_zone<'b>(&'b mut self, zone: ZE) -> impl Iterator<Item = &'a mut E> + 'b {
        let entity_service = self.entity_wrapper.get_mut();
        let zone_storage = self.zone_wrapper.get_mut();
        zone_storage
            .zone_assignment
            .get_mut(&zone)
            .map_or_else(|| [].iter_mut(), |z| z.iter_mut())
            // Unsafe precondition:
            // We are NOT allowed to created mutable reference aliases!
            // This means that only 1 mutable reference per Entity can be returned.
            // Fulfilling precondition:
            // All zones consist of a SET of EntityIds, also EntityService returns exactly
            // 1 and the same Entity for every provided ID.
            .map(move |e_id| unsafe { &mut *(entity_service.get_mut(*e_id).unwrap() as *mut E) })
    }
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
            entity_wrapper: MutSwitch::from_ref(service),
            zone_wrapper: MutSwitch::from_ref(&self.storage),
        }
    }
}

impl<'a, E, ZE> AdapterCompliantMut<'a, ZoneAdapter<'a, CTTrue, E, ZE>> for ZoneServiceStub<E, ZE>
where
    E: Entity + Identifiable<ID = EntityId> + Clone,
    ZE: ZoneEnumerator + Hash + Eq + Default,
{
    fn build_mut(&'a mut self, service: &'a mut EntityService<E>) -> ZoneAdapter<CTTrue, E, ZE> {
        ZoneAdapter {
            entity_wrapper: MutSwitch::from_mut(service),
            zone_wrapper: MutSwitch::from_mut(&mut self.storage),
        }
    }
}
