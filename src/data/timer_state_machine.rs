use crate::data::TimerContent;
use crate::prelude::*;
use crate::subclass::prelude::*;
use gtk::glib;

#[doc(hidden)]
mod imp {
    use gtk::glib::subclass::{Signal, SignalType};
    use once_cell::sync::Lazy;

    use super::*;

    #[derive(Clone, Copy)]
    #[repr(C)]
    pub struct TimerStateMachineInterface {
        pub type_iface: glib::gobject_ffi::GTypeInterface,

        pub press: fn(&super::TimerStateMachine),
        pub release: fn(&super::TimerStateMachine),
        pub press_timeout: fn(&super::TimerStateMachine),
        pub tick: fn(&super::TimerStateMachine),
        pub content: fn(&super::TimerStateMachine) -> TimerContent,
    }

    #[glib::object_interface]
    unsafe impl ObjectInterface for TimerStateMachineInterface {
        const NAME: &'static str = "PtTimerStateMachine";
        type Prerequisites = ();

        fn properties() -> &'static [glib::ParamSpec] {
            static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
                vec![
                    glib::ParamSpecBoolean::builder("finished").build(),
                    glib::ParamSpecBoolean::builder("running").build(),
                ]
            });

            PROPERTIES.as_ref()
        }

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
    /// The interface for timer state machines.
    pub struct TimerStateMachine(ObjectInterface<imp::TimerStateMachineInterface>);
}

/// Trait that contains defined method in `StateMachine`
pub trait TimerStateMachineExt: 'static {
    fn press(&self);
    fn release(&self);
    fn press_timeout(&self);
    fn tick(&self);
    fn content(&self) -> TimerContent;
}

impl<O: IsA<TimerStateMachine>> TimerStateMachineExt for O {
    fn press(&self) {
        let iface = self.interface::<TimerStateMachine>().unwrap();
        (iface.as_ref().press)(self.upcast_ref())
    }

    fn release(&self) {
        let iface = self.interface::<TimerStateMachine>().unwrap();
        (iface.as_ref().release)(self.upcast_ref())
    }

    fn press_timeout(&self) {
        let iface = self.interface::<TimerStateMachine>().unwrap();
        (iface.as_ref().press_timeout)(self.upcast_ref())
    }

    fn tick(&self) {
        let iface = self.interface::<TimerStateMachine>().unwrap();
        (iface.as_ref().tick)(self.upcast_ref())
    }

    fn content(&self) -> TimerContent {
        let iface = self.interface::<TimerStateMachine>().unwrap();
        (iface.as_ref().content)(self.upcast_ref())
    }
}

/// Trait that must be implemented by objects that implements `StateMachine`.
pub trait TimerStateMachineImpl: ObjectImpl {
    fn press(&self);
    fn release(&self);
    fn press_timeout(&self);
    fn tick(&self);
    fn content(&self) -> TimerContent;
}

unsafe impl<T> IsImplementable<T> for TimerStateMachine
where
    T: TimerStateMachineImpl,
    <T as ObjectSubclass>::Type: IsA<TimerStateMachine>,
{
    fn interface_init(iface: &mut glib::Interface<Self>) {
        let iface = iface.as_mut();

        iface.press = state_machine_press_trampoline::<T>;
        iface.release = state_machine_release_trampoline::<T>;
        iface.press_timeout = state_machine_press_timeout_trampoline::<T>;
        iface.tick = state_machine_tick_trampoline::<T>;
        iface.content = state_machine_content_trampoline::<T>;
    }
}

fn state_machine_press_trampoline<T>(state_machine: &TimerStateMachine)
where
    T: TimerStateMachineImpl,
    <T as ObjectSubclass>::Type: IsA<TimerStateMachine>,
{
    state_machine
        .downcast_ref::<T::Type>()
        .unwrap()
        .imp()
        .press();
}

fn state_machine_release_trampoline<T>(state_machine: &TimerStateMachine)
where
    T: TimerStateMachineImpl,
    <T as ObjectSubclass>::Type: IsA<TimerStateMachine>,
{
    state_machine
        .downcast_ref::<T::Type>()
        .unwrap()
        .imp()
        .release();
}

fn state_machine_press_timeout_trampoline<T>(state_machine: &TimerStateMachine)
where
    T: TimerStateMachineImpl,
    <T as ObjectSubclass>::Type: IsA<TimerStateMachine>,
{
    state_machine
        .downcast_ref::<T::Type>()
        .unwrap()
        .imp()
        .press_timeout();
}

fn state_machine_tick_trampoline<T>(state_machine: &TimerStateMachine)
where
    T: TimerStateMachineImpl,
    <T as ObjectSubclass>::Type: IsA<TimerStateMachine>,
{
    state_machine
        .downcast_ref::<T::Type>()
        .unwrap()
        .imp()
        .tick();
}

fn state_machine_content_trampoline<T>(state_machine: &TimerStateMachine) -> TimerContent
where
    T: TimerStateMachineImpl,
    <T as ObjectSubclass>::Type: IsA<TimerStateMachine>,
{
    state_machine
        .downcast_ref::<T::Type>()
        .unwrap()
        .imp()
        .content()
}
