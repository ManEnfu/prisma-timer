use std::mem;
use std::time::Duration;
use std::time::Instant;

use crate::data::{
    IsTimerState, Penalty, SolveTime, TimerContent, TimerState, TimerStateMachine, TimerStatePriv,
};
use crate::prelude::*;
use crate::subclass::prelude::*;
use gtk::glib;

const WAIT_TIMEOUT: u64 = 500;
const TICK_INTERVAL: u64 = 10;

const EXPECT_RWLOCK: &str = "Error accessing timer state.";

mod finished;
mod idle;
mod ready;
mod timing;
mod wait;

#[doc(hidden)]
mod imp {
    use std::{
        cell::{Cell, RefCell},
        marker::PhantomData,
        sync::RwLock,
    };

    use super::{idle::Idle, *};

    #[derive(Default, glib::Properties)]
    #[properties(wrapper_type = super::SimpleTimerStateMachine)]
    pub struct SimpleTimerStateMachine {
        pub(super) state: RwLock<TimerStatePriv>,
        pub(super) last_solve: RwLock<SolveTime>,
        pub(super) state_: RefCell<Option<Box<dyn IsTimerState>>>,
        pub(super) is_pressed: Cell<bool>,

        #[property(get = Self::is_finished, override_interface = TimerStateMachine)]
        finished: PhantomData<bool>,
        #[property(get = Self::is_running, override_interface = TimerStateMachine)]
        running: PhantomData<bool>,
    }

    impl SimpleTimerStateMachine {
        fn is_finished(&self) -> bool {
            self.state_
                .borrow()
                .as_ref()
                .map(|s| s.is_finished())
                .unwrap_or_default()
        }

        fn is_running(&self) -> bool {
            self.state_
                .borrow()
                .as_ref()
                .map(|s| s.is_running())
                .unwrap_or_default()
        }

        fn switch_state(&self, state: Option<Box<dyn IsTimerState>>) {
            let obj = self.obj();
            self.state_.replace(state);
            obj.emit_by_name::<()>("state-changed", &[]);
            obj.notify_running();
            obj.notify_finished();
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SimpleTimerStateMachine {
        const NAME: &'static str = "PtSimpleTimerStateMachine";
        type Type = super::SimpleTimerStateMachine;
        type Interfaces = (TimerStateMachine,);
    }

    #[glib::derived_properties]
    impl ObjectImpl for SimpleTimerStateMachine {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();
            self.state_
                .replace(Some(Box::new(Idle::new(Some(obj.as_ref())))));
        }
    }

    impl TimerStateMachineImpl for SimpleTimerStateMachine {
        fn press(&self) {
            if !self.is_pressed.get() {
                self.is_pressed.set(true);

                if let Some(state) = self.state_.take() {
                    let new_state = state.press();
                    self.switch_state(Some(new_state));
                }
            }
        }

        fn release(&self) {
            if self.is_pressed.get() {
                self.is_pressed.set(false);

                if let Some(state) = self.state_.take() {
                    let new_state = state.release();
                    self.switch_state(Some(new_state));
                }
            }
        }

        fn press_timeout(&self) {
            if let Some(state) = self.state_.take() {
                let new_state = state.press_timeout();
                self.switch_state(Some(new_state));
            }
        }

        fn tick(&self) {
            if let Some(state) = self.state_.take() {
                let new_state = state.tick();
                self.switch_state(Some(new_state));
            }
        }

        fn content(&self) -> TimerContent {
            self.state_
                .borrow()
                .as_ref()
                .map(|s| s.content())
                .unwrap_or_default()
        }
    }
}

glib::wrapper! {
    /// The state machine of a timer.
    pub struct SimpleTimerStateMachine(ObjectSubclass<imp::SimpleTimerStateMachine>)
        @implements TimerStateMachine;
}

impl SimpleTimerStateMachine {
    /// Creates a new state machine.
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    /// Gets the state of the machine.
    pub fn state(&self) -> TimerState {
        let imp = self.imp();
        imp.state.read().expect(EXPECT_RWLOCK).to_state()
    }

    /// Gets the last time recorded by the timer.
    pub fn last_solve(&self) -> SolveTime {
        let imp = self.imp();
        *imp.last_solve.read().expect(EXPECT_RWLOCK)
    }

    /// Called when timer trigger is pressed.
    pub(crate) fn press_cb(&self) {
        let imp = self.imp();
        let mut state_changed = true;

        {
            let mut state = imp.state.write().expect(EXPECT_RWLOCK);
            let o_state = mem::take(&mut *state);

            let n_state = match o_state {
                TimerStatePriv::Idle => {
                    let timeout = glib::timeout_add_local_once(
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
                        if plus_2 { Penalty::Plus2 } else { Penalty::Ok },
                    );
                    *imp.last_solve.write().expect(EXPECT_RWLOCK) = solve_time;
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
    pub(crate) fn release_cb(&self) {
        let imp = self.imp();
        let mut state_changed = true;

        {
            let mut state = imp.state.write().expect(EXPECT_RWLOCK);
            let o_state = mem::take(&mut *state);

            let n_state = match o_state {
                TimerStatePriv::Wait { timeout_id } => {
                    timeout_id.remove();
                    TimerStatePriv::Idle
                }
                TimerStatePriv::Ready => {
                    let tick_cb = glib::timeout_add_local(
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
    pub(crate) fn press_timeout_cb(&self) {
        let imp = self.imp();
        let mut state_changed = true;

        {
            let mut state = imp.state.write().expect(EXPECT_RWLOCK);
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
    pub(crate) fn tick_cb(&self) {
        let imp = self.imp();

        {
            let mut state = imp.state.write().expect(EXPECT_RWLOCK);

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

impl Default for SimpleTimerStateMachine {
    fn default() -> Self {
        Self::new()
    }
}
