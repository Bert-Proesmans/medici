use std::marker::PhantomData;

use crate::behaviours::{functions, markers, transitions};
use crate::compile_tools as ct;
use crate::wrappers::SimpleWrapper;

#[derive(Debug, Clone)]
pub struct MachineCore<State, Stack>
where
    State: markers::TopLevelState + functions::State,
    Stack: ct::Stack,
{
    pub state: PhantomData<State>,
    pub history: PhantomData<Stack>,
    pub transaction: State::Transaction,
}

impl<State, Stack> functions::StateMachine for MachineCore<State, Stack>
where
    State: markers::TopLevelState + functions::State,
    Stack: ct::Stack,
{
    type State = State;
    type TransitionRecord = Stack;
}

/*
where
    Previous: function::StateMachine + transition::Transition<Previous, Next, SimpleWrapper<Previous>> + Clone,
    Next: function::StateMachine,

    Container: function::MachineContainer<Previous> + function::MachineWrapper<Next>,
 */

/*
impl<Previous, Next, Container> transition::Transition<Previous, Next, Container> for Container
where
    Previous: function::StateMachine,
    Next: function::StateMachine,

    Container: function::MachineContainer<Previous> + function::MachineWrapper<Next>,
{
    fn transition(
        mut self,
        transaction: <Next::State as function::State>::Transaction,
    ) -> Container::Output {
        let tmp = self.get_core().clone();
        let core = std::mem::replace(self.get_core_mut(), tmp);
        let new_core = core.transition(transaction);
        self.wrap(new_core.unwrap())
    }
}
*/

/*
where
    Previous: function::StateMachine<TransitionRecord = CTS::Tail> + transition::Pushdown<Previous, Next, CTS, SimpleWrapper<Previous>> + Clone,
    Next: function::StateMachine,

    CTS: ct::Stack<Head = <Next as function::StateMachine>::State>,
    Container: function::MachineContainer<Previous> + function::MachineWrapper<Next>,
    <Container as function::MachineWrapper<Next>>::Output: function::MachineContainer<Next>,
 */

/*
impl<Previous, Next, CTS, Container> transition::Pushdown<Previous, Next, CTS, Container>
    for Container
where
    Previous: function::StateMachine,
    Next: function::StateMachine,

    Container: function::MachineContainer<Previous> + function::MachineWrapper<Next>,
    CTS: ct::Stack,
{
    fn pushdown(
        mut self,
        transaction: <Next::State as function::State>::Transaction,
    ) -> Container::Output {
        unimplemented!()
    }
}
*/
