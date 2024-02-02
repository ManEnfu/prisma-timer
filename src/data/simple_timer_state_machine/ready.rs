use gtk::glib;

use crate::{
    data::{IsTimerState, TimerContent, TimerContentColor, TimerStateMachine},
    prelude::*,
};

use super::timing::Timing;

pub struct Ready {
    state_machine: glib::WeakRef<TimerStateMachine>,
}

impl Ready {
    pub fn new(state_machine: Option<&impl IsA<TimerStateMachine>>) -> Self {
        let state_machine = state_machine.map(Cast::upcast_ref::<TimerStateMachine>);

        let weak_ref = glib::WeakRef::new();
        weak_ref.set(state_machine);

        Self {
            state_machine: weak_ref,
        }
    }
}

impl IsTimerState for Ready {
    fn release(self: Box<Self>) -> Box<dyn IsTimerState> {
        Box::new(Timing::new(self.state_machine.upgrade().as_ref(), false))
    }

    fn is_running(&self) -> bool {
        true
    }

    fn content(&self) -> TimerContent {
        TimerContent {
            value: None,
            color: TimerContentColor::Success,
        }
    }
}
