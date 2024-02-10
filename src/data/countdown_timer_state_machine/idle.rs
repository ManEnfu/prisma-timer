use gtk::glib;

use crate::{
    data::{IsTimerState, TimerContent, TimerContentColor, TimerStateMachine},
    prelude::*,
};

use super::idle_ready::IdleReady;

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
    fn noop(self: Box<Self>) -> Box<dyn IsTimerState> {
        self
    }

    fn press(self: Box<Self>) -> Box<dyn IsTimerState> {
        Box::new(IdleReady::new(self.state_machine.upgrade().as_ref()))
    }

    fn is_idle(&self) -> bool {
        true
    }

    fn content(&self) -> TimerContent {
        TimerContent {
            value: None,
            color: TimerContentColor::Neutral,
        }
    }
}
