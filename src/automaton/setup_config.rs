use std::default::Default;

const MAX_PLAYERS: i32 = 2;

pub struct SetupConfig {
	pub starting_player_ord: u32,
	pub player_names: [&'static str; MAX_PLAYERS as usize],
}

impl Default for SetupConfig {
    fn default() -> Self {
    	SetupConfig {
    		starting_player_ord: 1,
    		player_names: ["Player 1", "Player 2"],
    	}
    }
}
