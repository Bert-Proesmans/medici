extern crate game_rules;

// use game_rules::action::*;
use game_rules::game_system::prelude::*;
// use game_rules::trigger::*;

#[test]
#[should_panic]
fn max_entities() {
    let mut config: SetupConfig = Default::default();
    // Configure for 1 player.
    config.player_names = Default::default();
    config.player_names[0] = Some("Player 1".into());
    // Limit maximum entities to 1 (only the game entity).
    config.max_entities = 1;
    // TODO; Update when errors are properly linked!
    let machine = Machine::new(&config).unwrap();
    // If it DOESN'T panic, print all data.
    println!("Panic did NOT happen! Printing relevant structs");
    println!("{:?}", config);
    println!("{:?}", machine);
}

#[test]
fn config() {
    const NUM_PLAYERS: usize = 5;
    //
    let mut config: SetupConfig = Default::default();
    // Generates 0, 1, 2, 3, ..
    for i in 0..NUM_PLAYERS {
        let player_name: String = i.to_string();
        config.player_names[i] = Some(player_name);
    }
    config.max_entities = 50;
    let machine = Machine::new(&config).unwrap();
    //
    assert_eq!(GAME_E_ID, 0);
    let game_entity = machine.entities.get_entity(GAME_E_ID).unwrap();
    let max_players = game_entity.get_value(&EntityTags::MaxPlayers).unwrap();
    assert_eq!(NUM_PLAYERS, max_players as usize);
    // Check the name for each player entity
    for i in 0..NUM_PLAYERS {
        // Player id is 1-indexed, which matches perfectly on entity idx
        // because the game entity ALWAYS has entity id 0.
        let player_idx = i + 1;
        let player = machine.entities.get_entity(player_idx).unwrap();
        assert_eq!(player.human_readable, config.player_names[i]);
    }
}
