use crate::prelude::*;
use gtk::glib;

use super::wait::Wait;
use crate::data::{IsTimerState, TimerStateMachine};

pub struct Idle {
    state_machine: glib::WeakRef<TimerStateMachine>,
}

impl Idle {
    pub fn new(state_machine: Option<&impl IsA<TimerStateMachine>>) -> Self {
        let state_machine = state_machine.map(Cast::upcast_ref::<TimerStateMachine>);

        let weak_ref = glib::WeakRef::new();
        weak_ref.set(state_machine);

        Self {
            state_machine: weak_ref,
        }
    }
}

impl IsTimerState for Idle {
    fn press(self: Box<Self>) -> Box<dyn IsTimerState> {
        Box::new(Wait::new(self.state_machine.upgrade().as_ref()))
    }
}
