use crate::data::{SimpleTimerStateMachine, TimerStateMachine};
use crate::subclass::prelude::*;
use gtk::glib;

#[doc(hidden)]
mod imp {
    use super::*;

    #[derive(Default)]
    pub struct CountdownTimerStateMachine;

    #[glib::object_subclass]
    impl ObjectSubclass for CountdownTimerStateMachine {
        const NAME: &'static str = "PtCountdownTimerStateMachine";
        type Type = super::CountdownTimerStateMachine;
        type ParentType = SimpleTimerStateMachine;
    }

    impl ObjectImpl for CountdownTimerStateMachine {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }

    impl SimpleTimerStateMachineImpl for CountdownTimerStateMachine {}
}

glib::wrapper! {
    /// The state machine of a timer with inspection countdown.
    pub struct CountdownTimerStateMachine(ObjectSubclass<imp::CountdownTimerStateMachine>)
        @extends SimpleTimerStateMachine,
        @implements TimerStateMachine;
}

impl CountdownTimerStateMachine {
    /// Creates a new state machine.
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}
