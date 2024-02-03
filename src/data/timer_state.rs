use super::TimerContent;

pub trait IsTimerState {
    /// No-operation. Must be implemented by implementors by returning themselves.
    /// This is a workaround to make `IsTimerState` object safe.
    fn noop(self: Box<Self>) -> Box<dyn IsTimerState>;

    fn press(self: Box<Self>) -> Box<dyn IsTimerState> {
        log::warn!("transition `press` is not defined in this state.");
        self.noop()
    }

    fn release(self: Box<Self>) -> Box<dyn IsTimerState> {
        log::warn!("transition `release` is not defined in this state.");
        self.noop()
    }

    fn press_timeout(self: Box<Self>) -> Box<dyn IsTimerState> {
        log::warn!("transition `press_timeout` is not defined in this state.");
        self.noop()
    }

    fn tick(self: Box<Self>) -> Box<dyn IsTimerState> {
        log::warn!("transition `tick` is not defined in this state.");
        self.noop()
    }

    fn is_finished(&self) -> bool {
        false
    }

    fn is_running(&self) -> bool {
        false
    }

    fn content(&self) -> TimerContent {
        TimerContent::default()
    }
}
