use std::time::{Duration, Instant};

use crate::{
    data::{
        IsTimerState, Penalty, SolveTime, TimerContent, TimerContentColor, TimerContentValue,
        TimerStateMachine,
    },
    prelude::*,
};

use gtk::glib;

use super::finished::Finished;

const TICK_INTERVAL: u64 = 10;

pub struct Timing {
    state_machine: glib::WeakRef<TimerStateMachine>,
    tick_id: Option<glib::SourceId>,
    duration: Duration,
    last_tick: Instant,
    is_plus_2: bool,
}

impl Timing {
    pub fn new(state_machine: Option<&impl IsA<TimerStateMachine>>, is_plus_2: bool) -> Self {
        let state_machine = state_machine.map(Cast::upcast_ref::<TimerStateMachine>);

        let weak_ref = glib::WeakRef::new();
        weak_ref.set(state_machine);

        let tick_id = state_machine.map(|sm| {
            glib::timeout_add_local(
                Duration::from_millis(TICK_INTERVAL),
                glib::clone!(@weak sm => @default-return glib::ControlFlow::Continue, move || {
                    sm.tick();
                    glib::ControlFlow::Continue
                }),
            )
        });

        Self {
            state_machine: weak_ref,
            tick_id,
            last_tick: Instant::now(),
            duration: Duration::ZERO,
            is_plus_2,
        }
    }
}

impl Drop for Timing {
    fn drop(&mut self) {
        if let Some(tick_id) = self.tick_id.take() {
            tick_id.remove();
        }
    }
}

impl IsTimerState for Timing {
    fn press(self: Box<Self>) -> Box<dyn IsTimerState> {
        let solve_time = SolveTime::new(
            self.duration + (Instant::now() - self.last_tick),
            if self.is_plus_2 {
                Penalty::Plus2
            } else {
                Penalty::Ok
            },
        );
        Box::new(Finished::new(
            self.state_machine.upgrade().as_ref(),
            solve_time,
        ))
    }

    fn tick(mut self: Box<Self>) -> Box<dyn IsTimerState> {
        let new_tick = Instant::now();
        self.duration += new_tick - self.last_tick;
        self.last_tick = new_tick;
        self
    }

    fn is_running(&self) -> bool {
        true
    }

    fn content(&self) -> TimerContent {
        TimerContent {
            value: Some(TimerContentValue::SolveTime(SolveTime::new(
                self.duration,
                Penalty::Ok,
            ))),
            color: TimerContentColor::Neutral,
        }
    }
}
