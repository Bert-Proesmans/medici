#![feature(nll)]

extern crate game_rules;

use game_rules::action::*;
use game_rules::game_system::prelude::*;
use game_rules::trigger::*;

/*
#[test]
fn transition() {
    let config: SetupConfig = Default::default();
    let game: Machine![Wait<Start>] = Machine::new(&config).expect("Error creating new game!");
    let game: Machine![Action<Start>] = game.transitio(Epsilon).expect("Game enexpectedly finished");
    let game: Machine![Effect<Start>] = game.pushdown(Epsilon).expect("Game enexpectedly finished");
}
*/

#[test]
fn game_setup() {
    let config: SetupConfig = Default::default();
    let mut game = Machine::new(&config).expect("Error creating new game!");

    {
        let game_entity = game.entities.get(GAME_E_ID).unwrap();
        assert_eq!(GAME_E_ID, 0);
        assert_eq!(GAME_E_ID, game_entity.id());
    }

    // Add triggers.
    // These triggers are specialized to use AnyStack for the compile-time stack
    // generic parameter. This is allowed because the size of any CTStack within
    // our state machine is 0.
    game.triggers
        .add_trigger(start_game_trigger::<AnyStack>)
        .unwrap();
    game.triggers
        .add_trigger(turn_end_trigger::<AnyStack>)
        .unwrap();

    // Start game
    let first_turn = start_game(game).expect("Game unexpectedly finished");

    // Check we're currently within the turn of player 1.
    let game_entity = first_turn.entities.get(GAME_E_ID).unwrap();
    assert_eq!(
        game_entity.get_value_default(&EntityTags::CurrentPlayerOrd),
        1
    );
    let second_turn = end_turn(first_turn).expect("Game unexpectedly finished");

    // Check we're currently within the turn of player 2.
    let game_entity = second_turn.entities.get(GAME_E_ID).unwrap();
    assert_eq!(
        game_entity.get_value_default(&EntityTags::CurrentPlayerOrd),
        2
    );
    let _third_turn = end_turn(second_turn).expect("Game unexpectedly finished");
}
