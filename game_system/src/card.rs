//! Specialized implementation of the [`Card`] game-object.

use medici_core::prefab::card::CardStruct;

use state_machine::state::prelude::{TimingItem, TriggerItem};
use tag::EntityTags;

/// [Re-export] Identifier for referencing Game cards.
pub use medici_core::prefab::card::GAME_CARD_ID;
/// [Re-export] Identifier for referencing Player cards.
pub use medici_core::prefab::card::PLAYER_CARD_ID;

/// The specialized [`Card`] structure.
pub type Card = CardStruct<EntityTags, TimingItem, TriggerItem>;

#[macro_export]
/// Use this macro to build implementations of new cards.
macro_rules! card_impl {
    (
    	ID = $id:expr ; NAME = $name:expr ;
    	properties { $($prop_name:path = $prop_value:expr;)* }
    	triggers { $( $method:ident [ TIMING = $timing:path ; TRIGGER = $trigger:path ; ] =>
    		| $machine_name:ident | $method_impl:block )* }
	) => {{
		#[allow(unused_imports)]
    	// Specific usages of std items are necessary. Thankfully std is always in scope!
    	// std will be resolved at the calling site.
    	use std::fmt::Debug;
    	use $crate::prelude::CardBuilder;
		//
		let mut c = $crate::prelude::Card::new_with_id($id);
		c.name = $name;
		//
		$( c.set_value($prop_name, $prop_value); )*
		// A block argument already has opening and closing braces!
		$(
			fn $method<CTS>($machine_name: $crate::prelude::Machine<$crate::prelude::Trigger<$timing, $trigger>, CTS>) ->
			Result<$crate::prelude::Machine<$crate::prelude::Trigger<$timing, $trigger>, CTS>, $crate::prelude::MachineError>
			where
				CTS: $crate::prelude::CTStack + Debug + Clone + Send + 'static,
			{
				$method_impl
			}

			c.triggers.add_trigger($method::<AnyStack>);
		)*
		//
		c
    }};

    (
    	ID = $id:expr ; NAME = $name:expr ;
    	properties { $($prop_name:path = $prop_value:expr;)* }
	) => {
    	// Missing triggers
    	card_impl!{ ID = $id; NAME = $name; properties { $($prop_name = $prop_value;)*} triggers {}}
    };

    (
    	ID = $id:expr ; NAME = $name:expr ;
	) => {
    	// Missing properties + triggers
    	card_impl!{ ID = $id; NAME = $name; properties {} triggers {}}
    };
}

lazy_static! {
    /// Implementation of a card for a default game.
    static ref GAME_CARD: Card = {
        card_impl!{
            ID = GAME_CARD_ID;
            NAME = "Game card";
        }
    };

    /// Implementation of a card for a default player.
    static ref PLAYER_CARD: Card = {
        card_impl!{
            ID = PLAYER_CARD_ID;
            NAME = "Player card";

            properties {
                EntityTags::Health = 30;
                EntityTags::StartHandSize = 6;
            }
        }
    };
}
