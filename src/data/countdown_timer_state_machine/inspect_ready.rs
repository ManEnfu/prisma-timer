use std::time::{Duration, Instant};

use gtk::glib;

use crate::{
    data::{IsTimerState, TimerContent, TimerContentColor, TimerContentValue, TimerStateMachine},
    prelude::*,
};

use super::{dnf::Dnf, timing::Timing, PLUS_2_THRESHOLD};

pub struct InspectReady {
    state_machine: glib::WeakRef<TimerStateMachine>,
    tick_id: Option<glib::SourceId>,
    remaining_time: Duration,
    last_tick: Instant,
}

impl InspectReady {
    pub fn new_full(
        state_machine: Option<&impl IsA<TimerStateMachine>>,
        tick_id: Option<glib::SourceId>,
        remaining_time: Duration,
        last_tick: Instant,
    ) -> Self {
        let state_machine = state_machine.map(Cast::upcast_ref::<TimerStateMachine>);

        let weak_ref = glib::WeakRef::new();
        weak_ref.set(state_machine);

        Self {
            state_machine: weak_ref,
            tick_id,
            remaining_time,
            last_tick,
        }
    }
}

impl Drop for InspectReady {
    fn drop(&mut self) {
        if let Some(tick_id) = self.tick_id.take() {
            tick_id.remove();
        }
    }
}

impl IsTimerState for InspectReady {
    fn noop(self: Box<Self>) -> Box<dyn IsTimerState> {
        self
    }

    fn release(self: Box<Self>) -> Box<dyn IsTimerState> {
        Box::new(Timing::new(
            self.state_machine.upgrade().as_ref(),
            self.remaining_time < PLUS_2_THRESHOLD,
        ))
    }

    fn tick(mut self: Box<Self>) -> Box<dyn IsTimerState> {
        let new_tick = Instant::now();
        self.remaining_time = self
            .remaining_time
            .saturating_sub(new_tick - self.last_tick);

        if self.remaining_time.is_zero() {
            return Box::new(Dnf::new(self.state_machine.upgrade().as_ref()));
        }

        self.last_tick = new_tick;
        self
    }

    fn is_running(&self) -> bool {
        true
    }

    fn content(&self) -> TimerContent {
        let remaining_secs = self.remaining_time.as_secs();
        TimerContent {
            value: Some(TimerContentValue::String(if remaining_secs >= 2 {
                (remaining_secs - 1).to_string()
            } else {
                "+2".to_string()
            })),
            color: TimerContentColor::Success,
        }
    }
}
