use std::mem;
use std::sync::RwLockReadGuard;
use std::time::Duration;

use crate::data;
use adw::subclass::prelude::*;
use data::{TimerSimpleState, TimerState};
use gtk::glib;
use gtk::prelude::*;

const WAIT_TIMEOUT: u64 = 500;

mod imp {
    use std::sync::RwLock;

    use gtk::glib::subclass::{Signal, SignalType};
    use once_cell::sync::Lazy;

    use super::*;

    #[derive(Debug, Default)]
    pub struct TimerStateMachine {
        pub state: RwLock<TimerState>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TimerStateMachine {
        const NAME: &'static str = "PtTimerStateMachine";
        type Type = super::TimerStateMachine;
    }

    impl ObjectImpl for TimerStateMachine {
        fn constructed(&self) {
            self.parent_constructed();
        }

        fn signals() -> &'static [Signal] {
            static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
                vec![Signal::builder("state-changed")
                    .param_types(Vec::<SignalType>::new())
                    .build()]
            });
            SIGNALS.as_ref()
        }
    }
}

glib::wrapper! {
    pub struct TimerStateMachine(ObjectSubclass<imp::TimerStateMachine>);
}

impl TimerStateMachine {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn state(&self) -> RwLockReadGuard<'_, TimerState> {
        let imp = self.imp();
        imp.state.read().unwrap()
    }

    pub fn simple_state(&self) -> TimerSimpleState {
        let imp = self.imp();
        imp.state.read().unwrap().get_simple()
    }

    pub fn press(&self) {
        let imp = self.imp();
        let mut state_changed = true;

        {
            let mut state = imp.state.write().unwrap();
            let o_state = mem::take(&mut *state);

            let n_state = match o_state {
                TimerState::Idle => {
                    let timeout = glib::timeout_add_once(
                        Duration::from_millis(WAIT_TIMEOUT),
                        glib::clone!(@strong self as obj => move || {
                            obj.press_timeout();
                        }),
                    );
                    TimerState::Wait {
                        timeout_id: timeout,
                    }
                }
                TimerState::Timing => TimerState::Finished,
                s => {
                    state_changed = false;
                    s
                }
            };

            if state_changed {
                log::debug!("--press--> {:?}", &n_state)
            }
            let _ = mem::replace(&mut *state, n_state);
        }

        if state_changed {
            self.emit_by_name::<()>("state-changed", &[])
        }
    }

    pub fn release(&self) {
        let imp = self.imp();
        let mut state_changed = true;

        {
            let mut state = imp.state.write().unwrap();
            let o_state = mem::take(&mut *state);

            let n_state = match o_state {
                TimerState::Wait { timeout_id } => {
                    timeout_id.remove();
                    TimerState::Idle
                }
                TimerState::Ready => TimerState::Timing,
                TimerState::Finished => TimerState::Idle,
                s => {
                    state_changed = false;
                    s
                }
            };

            if state_changed {
                log::debug!("--release--> {:?}", &n_state)
            }
            let _ = mem::replace(&mut *state, n_state);
        }

        if state_changed {
            self.emit_by_name::<()>("state-changed", &[])
        }
    }

    pub fn press_timeout(&self) {
        let imp = self.imp();
        let mut state_changed = true;

        {
            let mut state = imp.state.write().unwrap();
            let o_state = mem::take(&mut *state);

            let n_state = match o_state {
                TimerState::Wait { timeout_id } => {
                    timeout_id.remove();
                    TimerState::Ready
                }
                s => {
                    state_changed = false;
                    s
                }
            };

            if state_changed {
                log::debug!("--timeout--> {:?}", &n_state)
            }
            let _ = mem::replace(&mut *state, n_state);
        }

        if state_changed {
            self.emit_by_name::<()>("state-changed", &[])
        }
    }
}

impl Default for TimerStateMachine {
    fn default() -> Self {
        Self::new()
    }
}
