//! Module containing zone definitions.

use medici_core::function::ZoneEnumerator;

#[derive(Debug)]
pub enum ZoneItem {
    SetAside,
    Deck,
    Play,
    Hand,
    // Chance,
    // Car,
    // FreePark,
}

impl ZoneEnumerator for ZoneItem {
    fn max_entities(&self) -> usize {
        match *self {
            ZoneItem::SetAside => usize::MAX,
            ZoneItem::Deck => 30,
            ZoneItem::Play => 7,
            ZoneItem::Hand => 10,
        }
    }
}
