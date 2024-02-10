use crate::data::TimerStateMachine;
use crate::prelude::*;
use crate::subclass::prelude::*;
use gtk::glib;

#[doc(hidden)]
mod imp {
    use once_cell::sync::Lazy;

    use super::*;

    #[derive(Clone, Copy)]
    #[repr(C)]
    pub struct TimerStateMachineProviderInterface {
        pub type_iface: glib::gobject_ffi::GTypeInterface,
    }

    #[glib::object_interface]
    unsafe impl ObjectInterface for TimerStateMachineProviderInterface {
        const NAME: &'static str = "PtTimerStateMachineProvider";
        type Prerequisites = (glib::Object,);

        fn properties() -> &'static [glib::ParamSpec] {
            static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| {
                vec![
                    glib::ParamSpecObject::builder::<TimerStateMachine>("timer-state-machine")
                        .build(),
                ]
            });

            PROPERTIES.as_ref()
        }
    }
}

glib::wrapper! {
    /// The inteface for objects that provide `TimerStateMachine`.
    pub struct TimerStateMachineProvider(ObjectInterface<imp::TimerStateMachineProviderInterface>);
}

/// Trait that contains defined method in `TimerStateMachineProvider`
pub trait TimerStateMachineProviderExt: 'static {
    fn timer_state_machine(&self) -> TimerStateMachine;
}

impl<O: IsA<TimerStateMachineProvider>> TimerStateMachineProviderExt for O {
    fn timer_state_machine(&self) -> TimerStateMachine {
        self.property("timer-state-machine")
    }
}

/// Trait that must be implemented by objects that implements `TimerStateMachineProvider`.
pub trait TimerStateMachineProviderImpl: ObjectImpl {}

unsafe impl<T> IsImplementable<T> for TimerStateMachineProvider
where
    T: TimerStateMachineProviderImpl,
    <T as ObjectSubclass>::Type: IsA<TimerStateMachineProvider>,
{
}
