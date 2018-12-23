pub mod states;
pub mod transactions;
pub mod transitions;

use medici_core::behaviours::functions;

pub struct HearthStone<Core>
where
    Core: functions::StateMachine,
{
    _core: Core,

    // STUB
    pub random_state: u32,
}

impl<Core, M> functions::MachineWrapper<M> for HearthStone<Core>
where
    M: functions::StateMachine,
    Core: functions::StateMachine,
{
    type Output = HearthStone<M>;

    fn wrap(self, machine: M) -> Self::Output {
        HearthStone {
            _core: machine,
            random_state: 0,
        }
    }
}

impl<Core> functions::MachineContainer for HearthStone<Core>
where
    Core: functions::StateMachine,
{
    type Machine = Core;

    fn get_core(&self) -> &Self::Machine {
        &self._core
    }

    fn get_core_mut(&mut self) -> &mut Self::Machine {
        &mut self._core
    }
}
