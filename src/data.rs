use std::time::Duration;

pub use countdown_timer_state_machine::CountdownTimerStateMachine;
pub use penalty::Penalty;
pub use session::Session;
pub use session_item::SessionItem;
pub use simple_timer_state_machine::{
    SimpleTimerStateMachine, SimpleTimerStateMachineExt, SimpleTimerStateMachineImpl,
};
pub use solve_data::SolveData;
pub use solve_time::SolveTime;
pub use statistics::SolveStatistic;
pub use timer_content::{TimerContent, TimerContentColor, TimerContentValue};
pub use timer_settings::TimerSettings;
pub use timer_state::IsTimerState;
pub use timer_state_machine::{TimerStateMachine, TimerStateMachineExt, TimerStateMachineImpl};
pub use timer_state_machine_provider::{
    TimerStateMachineProvider, TimerStateMachineProviderExt, TimerStateMachineProviderImpl,
};

mod countdown_timer_state_machine;
mod penalty;
mod session;
mod session_item;
mod simple_timer_state_machine;
mod solve_data;
mod solve_time;
mod statistics;
mod timer_content;
mod timer_settings;
mod timer_state;
mod timer_state_machine;
mod timer_state_machine_provider;

pub const TICK_INTERVAL: Duration = Duration::from_millis(10);
pub const INSPECTION_TIME: Duration = Duration::from_secs(17);
pub const PLUS_2_THRESHOLD: Duration = Duration::from_secs(2);
pub const WAIT_TIMEOUT: Duration = Duration::from_millis(500);
