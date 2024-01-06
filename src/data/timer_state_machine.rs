use std::mem;
use std::sync::RwLockReadGuard;
use std::time::Duration;
use std::time::Instant;

use crate::data::{Penalty, SolveTime, TimerState, TimerStatePriv};
use adw::subclass::prelude::*;
use gtk::glib;
use gtk::prelude::*;

const WAIT_TIMEOUT: u64 = 500;
const TICK_INTERVAL: u64 = 10;

mod imp {
    use std::sync::RwLock;

    use gtk::glib::subclass::{Signal, SignalType};
    use once_cell::sync::Lazy;

    use super::*;

    #[derive(Debug, Default)]
    pub struct TimerStateMachine {
        pub state: RwLock<TimerStatePriv>,
        pub last_solve: RwLock<SolveTime>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TimerStateMachine {
        const NAME: &'static str = "PtTimerStateMachine";
        type Type = super::TimerStateMachine;
    }

    impl ObjectImpl for TimerStateMachine {
        fn signals() -> &'static [Signal] {
            static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
                vec![
                    Signal::builder("state-changed")
                        .param_types(Vec::<SignalType>::new())
                        .build(),
                    Signal::builder("tick")
                        .param_types(Vec::<SignalType>::new())
                        .build(),
                ]
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

    pub fn state(&self) -> RwLockReadGuard<'_, TimerStatePriv> {
        let imp = self.imp();
        imp.state.read().unwrap()
    }

    pub fn simple_state(&self) -> TimerState {
        let imp = self.imp();
        imp.state.read().unwrap().get_simple()
    }

    pub fn last_solve(&self) -> SolveTime {
        let imp = self.imp();
        *imp.last_solve.read().unwrap()
    }

    /// Called when timer trigger is pressed.
    pub fn press(&self) {
        let imp = self.imp();
        let mut state_changed = true;

        {
            let mut state = imp.state.write().unwrap();
            let o_state = mem::take(&mut *state);

            let n_state = match o_state {
                TimerStatePriv::Idle => {
                    let timeout = glib::timeout_add_once(
                        Duration::from_millis(WAIT_TIMEOUT),
                        glib::clone!(@weak self as obj => move || {
                            obj.press_timeout();
                        }),
                    );
                    TimerStatePriv::Wait {
                        timeout_id: timeout,
                    }
                }
                TimerStatePriv::Timing {
                    last_tick,
                    tick_cb_id,
                    duration,
                    plus_2,
                } => {
                    tick_cb_id.remove();
                    let solve_time = SolveTime::new(
                        duration + (Instant::now() - last_tick),
                        if plus_2 { Some(Penalty::Plus2) } else { None },
                    );
                    *imp.last_solve.write().unwrap() = solve_time;
                    TimerStatePriv::Finished { solve_time }
                }
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

    /// Called when timer trigger is released.
    pub fn release(&self) {
        let imp = self.imp();
        let mut state_changed = true;

        {
            let mut state = imp.state.write().unwrap();
            let o_state = mem::take(&mut *state);

            let n_state = match o_state {
                TimerStatePriv::Wait { timeout_id } => {
                    timeout_id.remove();
                    TimerStatePriv::Idle
                }
                TimerStatePriv::Ready => {
                    let tick_cb = glib::timeout_add(
                        Duration::from_millis(TICK_INTERVAL),
                        glib::clone!(@strong self as obj => move || {
                            obj.tick();
                            return glib::ControlFlow::Continue;
                        }),
                    );
                    TimerStatePriv::Timing {
                        last_tick: Instant::now(),
                        tick_cb_id: tick_cb,
                        duration: Duration::ZERO,
                        plus_2: false,
                    }
                }
                TimerStatePriv::Finished { .. } => TimerStatePriv::Idle,
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

    /// Called when duration of a trigger press exceeds certain threshold.
    pub fn press_timeout(&self) {
        let imp = self.imp();
        let mut state_changed = true;

        {
            let mut state = imp.state.write().unwrap();
            let o_state = mem::take(&mut *state);

            let n_state = match o_state {
                TimerStatePriv::Wait { timeout_id } => {
                    timeout_id.remove();
                    TimerStatePriv::Ready
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

    /// Called on every tick during `Timing` state.
    pub fn tick(&self) {
        let imp = self.imp();

        {
            let mut state = imp.state.write().unwrap();

            if let TimerStatePriv::Timing {
                last_tick,
                duration,
                ..
            } = &mut *state
            {
                let new_tick = Instant::now();
                *duration += new_tick - *last_tick;
                *last_tick = new_tick;
            }
        }

        self.emit_by_name::<()>("tick", &[])
    }
}

impl Default for TimerStateMachine {
    fn default() -> Self {
        Self::new()
    }
}
