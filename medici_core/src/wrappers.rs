use crate::behaviours::functions;

pub struct SimpleWrapper<Core: functions::StateMachine>(Core);

impl<Core: functions::StateMachine> SimpleWrapper<Core> {
    pub fn new(machine: Core) -> Self {
        SimpleWrapper(machine)
    }

    pub(crate) fn unwrap(self) -> Core {
        self.0
    }
}

impl<Previous, Next> functions::MachineWrapper<Next> for SimpleWrapper<Previous>
where
    Previous: functions::StateMachine,
    Next: functions::StateMachine,
{
    type Output = SimpleWrapper<Next>;
    fn wrap(self, machine: Next) -> Self::Output {
        SimpleWrapper(machine)
    }
}

impl<Core> functions::MachineContainer for SimpleWrapper<Core>
where
    Core: functions::StateMachine,
{
    type Machine = Core;

    fn get_core(&self) -> &Self::Machine {
        &self.0
    }

    fn get_core_mut(&mut self) -> &mut Self::Machine {
        &mut self.0
    }
}
