use std::time::{Duration, Instant};

use gtk::glib;

use super::SolveTime;

/// The representation of the state of a timer.
#[derive(Debug, Default)]
pub enum TimerStatePriv {
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
    pub fn get_simple(&self) -> TimerState {
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
    #[default]
    Idle,
    Wait,
    Ready,
    Timing {
        duration: Duration,
    },
    Finished {
        solve_time: SolveTime,
    },
}
