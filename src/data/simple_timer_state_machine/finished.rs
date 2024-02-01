use crate::{data::SolveTime, prelude::*};
use gtk::glib;

use super::idle::Idle;
use crate::data::{IsTimerState, TimerStateMachine};

pub struct Finished {
    state_machine: glib::WeakRef<TimerStateMachine>,
    solve_time: SolveTime,
}

impl Finished {
    pub fn new(state_machine: Option<&impl IsA<TimerStateMachine>>, solve_time: SolveTime) -> Self {
        let state_machine = state_machine.map(Cast::upcast_ref::<TimerStateMachine>);

        let weak_ref = glib::WeakRef::new();
        weak_ref.set(state_machine);

        Self {
            state_machine: weak_ref,
            solve_time,
        }
    }
}

impl IsTimerState for Finished {
    fn release(self: Box<Self>) -> Box<dyn IsTimerState> {
        Box::new(Idle::new(self.state_machine.upgrade().as_ref()))
    }

    fn is_finished(&self) -> bool {
        true
    }
}
