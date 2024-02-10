use gtk::glib;

use crate::{
    data::{
        IsTimerState, SolveTime, TimerContent, TimerContentColor, TimerContentValue,
        TimerStateMachine, WAIT_TIMEOUT,
    },
    prelude::*,
};

use super::idle::Idle;

pub struct Dnf {
    state_machine: glib::WeakRef<TimerStateMachine>,
    timeout_id: Option<glib::SourceId>,
}

impl Dnf {
    pub fn new(state_machine: Option<&impl IsA<TimerStateMachine>>) -> Self {
        let state_machine = state_machine.map(Cast::upcast_ref::<TimerStateMachine>);

        let weak_ref = glib::WeakRef::new();
        weak_ref.set(state_machine);

        let timeout_id = state_machine.map(|sm| {
            glib::timeout_add_local_once(
                WAIT_TIMEOUT,
                glib::clone!(@weak sm => move || {
                    sm.tick();
                }),
            )
        });

        Self {
            state_machine: weak_ref,
            timeout_id,
        }
    }
}

impl Drop for Dnf {
    fn drop(&mut self) {
        if let Some(timeout_id) = self.timeout_id.take() {
            timeout_id.remove();
        }
    }
}

impl IsTimerState for Dnf {
    fn noop(self: Box<Self>) -> Box<dyn IsTimerState> {
        self
    }

    fn tick(self: Box<Self>) -> Box<dyn IsTimerState> {
        Box::new(Idle::new(self.state_machine.upgrade().as_ref()))
    }

    fn is_finished(&self) -> bool {
        true
    }

    fn content(&self) -> TimerContent {
        TimerContent {
            value: Some(TimerContentValue::SolveTime(SolveTime::DNF)),
            color: TimerContentColor::Destructive,
        }
    }
}
