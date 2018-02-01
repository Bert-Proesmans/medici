use automaton::prelude::*;

pub fn turn_end_trigger(
    x: Game<Trigger<Peri, EndTurn>>,
) -> Result<Game<Trigger<Peri, EndTurn>>, Game<Finished>> {
    println!("[TURN_END_TRIGGER] PERI - END TURN");
    Ok(x)
}
