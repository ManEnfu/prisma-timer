use std::time::Duration;

use gtk::glib;

use crate::{
    data::{IsTimerState, TimerContent, TimerContentColor, TimerStateMachine},
    prelude::*,
};

use super::{idle::Idle, ready::Ready};

const WAIT_TIMEOUT: u64 = 500;

pub struct Wait {
    state_machine: glib::WeakRef<TimerStateMachine>,
    timeout_id: Option<glib::SourceId>,
}

impl Wait {
    pub fn new(state_machine: Option<&impl IsA<TimerStateMachine>>) -> Self {
        let state_machine = state_machine.map(Cast::upcast_ref::<TimerStateMachine>);

        let weak_ref = glib::WeakRef::new();
        weak_ref.set(state_machine);

        let timeout_id = state_machine.map(|sm| {
            glib::timeout_add_local_once(
                Duration::from_millis(WAIT_TIMEOUT),
                glib::clone!(@weak sm => move || {
                    sm.press_timeout();
                }),
            )
        });

        Self {
            state_machine: weak_ref,
            timeout_id,
        }
    }
}

impl Drop for Wait {
    fn drop(&mut self) {
        if let Some(timeout_id) = self.timeout_id.take() {
            timeout_id.remove();
        }
    }
}

impl IsTimerState for Wait {
    fn noop(self: Box<Self>) -> Box<dyn IsTimerState> {
        self
    }

    fn release(self: Box<Self>) -> Box<dyn IsTimerState> {
        Box::new(Idle::new(self.state_machine.upgrade().as_ref()))
    }

    fn press_timeout(self: Box<Self>) -> Box<dyn IsTimerState> {
        Box::new(Ready::new(self.state_machine.upgrade().as_ref()))
    }

    fn content(&self) -> TimerContent {
        TimerContent {
            value: None,
            color: TimerContentColor::Destructive,
        }
    }
}
