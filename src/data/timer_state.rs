use gtk::glib;

/// The representation of the state of a timer.
#[derive(Debug, Default)]
pub enum TimerState {
    #[default]
    Idle,
    Wait {
        timeout_id: glib::SourceId,
    },
    Ready,
    Timing,
    Finished,
}

impl TimerState {
    pub fn get_simple(&self) -> TimerSimpleState {
        match self {
            Self::Idle => TimerSimpleState::Idle,
            Self::Wait { timeout_id: _ } => TimerSimpleState::Wait,
            Self::Ready => TimerSimpleState::Ready,
            Self::Timing => TimerSimpleState::Timing,
            Self::Finished => TimerSimpleState::Finished,
        }
    }
}

/// The transferrable representation of the state of a timer.
#[derive(Debug, Default, Clone, Copy)]
pub enum TimerSimpleState {
    #[default]
    Idle,
    Wait,
    Ready,
    Timing,
    Finished,
}
