use std::time::Duration;

use crate::data::{SimpleTimerStateMachine, TimerStateMachine};
use crate::prelude::*;
use crate::subclass::prelude::*;
use gtk::glib;

mod dnf;
mod finished;
mod idle;
mod idle_ready;
mod inspect;
mod inspect_ready;
mod inspect_wait;
mod timing;

const TICK_INTERVAL: Duration = Duration::from_millis(10);
const INSPECTION_TIME: Duration = Duration::from_secs(17);
const PLUS_2_THRESHOLD: Duration = Duration::from_secs(2);
const WAIT_TIMEOUT: Duration = Duration::from_millis(500);

#[doc(hidden)]
mod imp {
    use super::{idle::Idle, *};

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

            let obj = self.obj();
            obj.set_state(Some(Box::new(Idle::new(Some(obj.as_ref())))));
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

impl Default for CountdownTimerStateMachine {
    fn default() -> Self {
        Self::new()
    }
}
