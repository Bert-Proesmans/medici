use medici_core::behaviours::transitions;
use medici_core::compile_tools as ct;
use medici_core::components::MachineCore;
use medici_core::wrappers::SimpleWrapper;

use crate::state_machine::states::{Empty, Start, Stop};
use crate::state_machine::HearthStone;

impl<CTS>
    transitions::Transition<
        MachineCore<Start, CTS>,
        MachineCore<Stop, CTS>,
        SimpleWrapper<MachineCore<Stop, CTS>>,
    > for MachineCore<Start, CTS>
where
    CTS: ct::Stack,
{
    /// Transition from the provided state into the implementing state.
    fn transition(self, t: Empty) -> SimpleWrapper<Stop> {
        unimplemented!()
    }
}
