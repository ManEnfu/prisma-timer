use std::time::{Duration, Instant};

use gtk::glib;

use super::SolveTime;

/// The internal representation of the state of a timer.
#[derive(Debug, Default)]
pub(crate) enum TimerStatePriv {
    #[default]
    Idle,
    Wait {
        timeout_id: glib::SourceId,
    },
    Ready,
    Timing {
        last_tick: Instant,
        tick_cb_id: glib::SourceId,
        duration: Duration,
        plus_2: bool,
    },
    Finished {
        solve_time: SolveTime,
    },
}

impl TimerStatePriv {
    pub fn to_state(&self) -> TimerState {
        match self {
            Self::Idle => TimerState::Idle,
            Self::Wait { .. } => TimerState::Wait,
            Self::Ready => TimerState::Ready,
            Self::Timing { duration, .. } => TimerState::Timing {
                duration: *duration,
            },
            Self::Finished { solve_time } => TimerState::Finished {
                solve_time: *solve_time,
            },
        }
    }
}

/// The transferrable representation of the state of a timer.
#[derive(Debug, Default, Clone, Copy)]
pub enum TimerState {
    /// The timer is idle.
    #[default]
    Idle,
    /// The timer is being pressed and is waiting for a period of time before
    /// switching to `Ready`
    Wait,
    /// The timer is ready to start.
    Ready,
    /// The timer is currently running.
    Timing { duration: Duration },
    /// The timer has finished timing.
    Finished { solve_time: SolveTime },
}
