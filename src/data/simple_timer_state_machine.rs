use crate::data::{IsTimerState, TimerContent, TimerStateMachine};
use crate::prelude::*;
use crate::subclass::prelude::*;
use gtk::glib;

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
    };

    use super::{idle::Idle, *};

    pub struct SimpleTimerStateMachineClass {
        pub parent_class: glib::object::ObjectClass,
    }

    unsafe impl ClassStruct for SimpleTimerStateMachineClass {
        type Type = SimpleTimerStateMachine;
    }

    #[derive(Default, glib::Properties)]
    #[properties(wrapper_type = super::SimpleTimerStateMachine)]
    pub struct SimpleTimerStateMachine {
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
        type Class = SimpleTimerStateMachineClass;
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
}

impl Default for SimpleTimerStateMachine {
    fn default() -> Self {
        Self::new()
    }
}

pub trait SimpleTimerStateMachineExt: 'static {}

impl<O: IsA<SimpleTimerStateMachine>> SimpleTimerStateMachineExt for O {}

pub trait SimpleTimerStateMachineImpl: ObjectImpl {}

unsafe impl<T> IsSubclassable<T> for SimpleTimerStateMachine
where
    T: SimpleTimerStateMachineImpl,
    <T as ObjectSubclass>::Type: IsA<SimpleTimerStateMachine>,
{
}
