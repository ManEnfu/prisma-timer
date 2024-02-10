use crate::data::{
    CountdownTimerStateMachine, SimpleTimerStateMachine, TimerStateMachine,
    TimerStateMachineProvider,
};
use crate::prelude::*;
use crate::subclass::prelude::*;
use gtk::{gio, glib};

#[doc(hidden)]
mod imp {
    use std::marker::PhantomData;

    use crate::config;

    use super::*;

    #[derive(Debug, glib::Properties)]
    #[properties(wrapper_type = super::TimerStateMachineSettingsProvider)]
    pub struct TimerStateMachineSettingsProvider {
        pub(super) settings: gio::Settings,

        pub(super) simple_timer_state_machine: SimpleTimerStateMachine,
        pub(super) countdown_timer_state_machine: CountdownTimerStateMachine,

        #[property(get = Self::timer_state_machine, override_interface = TimerStateMachineProvider)]
        timer_state_machine: PhantomData<TimerStateMachine>,
    }

    impl TimerStateMachineSettingsProvider {
        fn timer_state_machine(&self) -> TimerStateMachine {
            if self.settings.boolean("timer-use-countdown") {
                self.countdown_timer_state_machine.clone().upcast()
            } else {
                self.simple_timer_state_machine.clone().upcast()
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TimerStateMachineSettingsProvider {
        const NAME: &'static str = "PtTimerStateMachineSettingsProvider";
        type Type = super::TimerStateMachineSettingsProvider;
        type Interfaces = (TimerStateMachineProvider,);

        fn new() -> Self {
            Self {
                settings: gio::Settings::new(config::APP_ID),
                simple_timer_state_machine: SimpleTimerStateMachine::new(),
                countdown_timer_state_machine: CountdownTimerStateMachine::new(),
                timer_state_machine: PhantomData,
            }
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for TimerStateMachineSettingsProvider {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();

            self.settings.connect_changed(
                Some("timer-use-countdown"),
                glib::clone!(@weak obj => move |_, _| {
                    obj.notify_timer_state_machine();
                }),
            );
        }
    }

    impl TimerStateMachineProviderImpl for TimerStateMachineSettingsProvider {}
}

glib::wrapper! {
    /// A provider that provides `TimerStateMachine` based on 
    /// application settings.
    pub struct TimerStateMachineSettingsProvider(ObjectSubclass<imp::TimerStateMachineSettingsProvider>)
        @implements TimerStateMachineProvider;
}

impl TimerStateMachineSettingsProvider {
    pub fn new() -> Self {
        glib::Object::new()
    }
}

impl Default for TimerStateMachineSettingsProvider {
    fn default() -> Self {
        Self::new()
    }
}
