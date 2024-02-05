use super::SolveTime;

/// Content that can be shown by a timer interface.
#[derive(Debug, Default, Clone)]
pub struct TimerContent {
    pub value: Option<TimerContentValue>,
    pub color: TimerContentColor,
}

#[derive(Debug, Clone)]
pub enum TimerContentValue {
    SolveTime(SolveTime),
    Int(i32),
    String(String),
}

impl std::fmt::Display for TimerContentValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimerContentValue::SolveTime(solve_time) => solve_time.fmt(f),
            TimerContentValue::Int(i) => i.fmt(f),
            TimerContentValue::String(s) => s.fmt(f),
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum TimerContentColor {
    #[default]
    Neutral,
    Destructive,
    Warning,
    Success,
}
