use containers::games::Game;
use hs_automaton::states::*;

pub fn turn_end_trigger(
    x: Game<Trigger<Peri, EndTurn>>,
) -> Result<Game<Trigger<Peri, EndTurn>>, Game<Finished>> {
    println!("PERI - END TURN");
    Ok(x)
}
