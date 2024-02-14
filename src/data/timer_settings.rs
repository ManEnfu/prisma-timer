use crate::data::{
    CountdownTimerStateMachine, SimpleTimerStateMachine, TimerStateMachine,
    TimerStateMachineProvider,
};
use crate::prelude::*;
use crate::subclass::prelude::*;
use gtk::{gio, glib};

#[doc(hidden)]
#[allow(clippy::enum_variant_names)]
mod imp {
    use std::marker::PhantomData;

    use crate::config;

    use super::*;

    #[derive(Debug, glib::Properties)]
    #[properties(wrapper_type = super::TimerSettings)]
    pub struct TimerSettings {
        pub(super) settings: gio::Settings,

        pub(super) simple_timer_state_machine: SimpleTimerStateMachine,
        pub(super) countdown_timer_state_machine: CountdownTimerStateMachine,

        #[property(get = Self::timer_touch_only)]
        timer_touch_only: PhantomData<bool>,
        #[property(get = Self::timer_use_countdown)]
        timer_use_countdown: PhantomData<bool>,
        #[property(get = Self::timer_state_machine, override_interface = TimerStateMachineProvider)]
        timer_state_machine: PhantomData<TimerStateMachine>,
    }

    impl TimerSettings {
        fn timer_use_countdown(&self) -> bool {
            self.settings.boolean("timer-use-countdown")
        }

        fn timer_touch_only(&self) -> bool {
            self.settings.boolean("timer-touch-only")
        }

        fn timer_state_machine(&self) -> TimerStateMachine {
            if self.settings.boolean("timer-use-countdown") {
                self.countdown_timer_state_machine.clone().upcast()
            } else {
                self.simple_timer_state_machine.clone().upcast()
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TimerSettings {
        const NAME: &'static str = "PtTimerSettings";
        type Type = super::TimerSettings;
        type Interfaces = (TimerStateMachineProvider,);

        fn new() -> Self {
            Self {
                settings: gio::Settings::new(config::APP_ID),
                simple_timer_state_machine: SimpleTimerStateMachine::new(),
                countdown_timer_state_machine: CountdownTimerStateMachine::new(),
                timer_state_machine: PhantomData,
                timer_use_countdown: PhantomData,
                timer_touch_only: PhantomData,
            }
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for TimerSettings {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();

            self.settings.connect_changed(
                Some("timer-touch-only"),
                glib::clone!(@weak obj => move |_, _| {
                    obj.notify_timer_touch_only();
                }),
            );

            self.settings.connect_changed(
                Some("timer-use-countdown"),
                glib::clone!(@weak obj => move |_, _| {
                    obj.notify_timer_use_countdown();
                    obj.notify_timer_state_machine();
                }),
            );
        }
    }

    impl TimerStateMachineProviderImpl for TimerSettings {}
}

glib::wrapper! {
    /// A settings provider for timer. it also provides `TimerStateMachine`
    /// based on application settings.
    pub struct TimerSettings(ObjectSubclass<imp::TimerSettings>)
        @implements TimerStateMachineProvider;
}

impl TimerSettings {
    pub fn new() -> Self {
        glib::Object::new()
    }
}

impl Default for TimerSettings {
    fn default() -> Self {
        Self::new()
    }
}
