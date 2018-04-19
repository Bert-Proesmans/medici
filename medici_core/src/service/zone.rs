//! Module containing structures for working with zones.
//!
//! Zones are groups of entities. Each entity can only occur
//! in ONE zone at a time.

/*
#[derive(Debug)]
struct Zone<'a, E, I, EZ>
where
    E: Entity + 'a,
    <E as Entity>::ID: Display + Debug,
    I: Iterator<Item = &'a E>,
    EZ: ZoneEnumerator + Default,
{
    kind: EZ,
    limit: usize,
    entities: I,
}
*/
