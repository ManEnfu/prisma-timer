use std::time::Duration;

use crate::data::{self, TimerState};
use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gdk, glib};

mod imp {
    use std::cell::RefCell;

    use crate::util::TemplateCallbacks;

    use super::*;

    use gtk::glib::subclass::{Signal, SignalType};
    use once_cell::sync::Lazy;

    #[derive(Debug, Default, gtk::CompositeTemplate, glib::Properties)]
    #[template(resource = "/io/github/manenfu/PrismaTimer/ui/timer_face.ui")]
    #[properties(wrapper_type = super::TimerFace)]
    pub struct TimerFace {
        #[template_child]
        pub minutes: TemplateChild<gtk::Label>,
        #[template_child]
        pub colon: TemplateChild<gtk::Label>,
        #[template_child]
        pub seconds: TemplateChild<gtk::Label>,
        #[template_child]
        pub point: TemplateChild<gtk::Label>,
        #[template_child]
        pub centis: TemplateChild<gtk::Label>,

        #[template_child]
        pub statistics_box: TemplateChild<gtk::Box>,
        #[template_child]
        pub last_ao5_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub last_ao12_label: TemplateChild<gtk::Label>,

        #[property(get, set = Self::set_timer_state_machine)]
        pub timer_state_machine: RefCell<Option<data::TimerStateMachine>>,
        timer_state_machine_handlers: RefCell<Vec<glib::SignalHandlerId>>,

        #[property(get, set)]
        pub session: RefCell<Option<data::Session>>,
    }

    impl TimerFace {
        fn set_timer_state_machine(&self, v: Option<data::TimerStateMachine>) {
            let obj = self.obj();
            let mut handlers = self.timer_state_machine_handlers.borrow_mut();

            if let Some(osm) = self.timer_state_machine.take() {
                for id in handlers.drain(..) {
                    osm.disconnect(id);
                }
            }

            if let Some(sm) = &v {
                handlers.push(sm.connect_closure(
                    "state-changed",
                    false,
                    glib::closure_local!(@strong obj => move |sm: &data::TimerStateMachine| {
                        obj.timer_state_changed_cb(sm.simple_state());
                    }),
                ));
                handlers.push(sm.connect_closure(
                    "tick",
                    false,
                    glib::closure_local!(@strong obj => move |sm: &data::TimerStateMachine| {
                        obj.tick_cb(sm);
                    }),
                ));
            }
            self.timer_state_machine.replace(v);
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TimerFace {
        const NAME: &'static str = "PtTimerFace";
        type Type = super::TimerFace;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
            TemplateCallbacks::bind_template_callbacks(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for TimerFace {
        fn signals() -> &'static [Signal] {
            static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
                vec![
                    Signal::builder("timer-start")
                        .param_types(Vec::<SignalType>::new())
                        .build(),
                    Signal::builder("timer-stop")
                        .param_types(Vec::<SignalType>::new())
                        .build(),
                    Signal::builder("timer-ready")
                        .param_types(Vec::<SignalType>::new())
                        .build(),
                ]
            });
            SIGNALS.as_ref()
        }

        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            obj.add_css_class("timer-face");

            obj.set_time_label(Duration::ZERO);
            obj.setup_event_controllers();
            obj.setup_callbacks();
        }
    }
    impl WidgetImpl for TimerFace {}
    impl BinImpl for TimerFace {}
}

glib::wrapper! {
    pub struct TimerFace(ObjectSubclass<imp::TimerFace>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

#[gtk::template_callbacks]
impl TimerFace {
    fn setup_event_controllers(&self) {
        let key_events = gtk::EventControllerKey::new();
        key_events.connect_key_pressed(glib::clone!(@weak self as obj => @default-return glib::Propagation::Proceed, move |_, key, _, modifier| {
            if modifier.is_empty() && key == gdk::Key::space {
                obj.pressed_cb();
                glib::Propagation::Stop
            } else {
                glib::Propagation::Proceed
            }
        }));
        key_events.connect_key_released(
            glib::clone!(@weak self as obj => move |_, key, _, modifier| {
                if modifier.is_empty() && key == gdk::Key::space {
                    obj.released_cb();
                }
            }),
        );
        self.add_controller(key_events);

        let gestures = gtk::GestureClick::new();
        gestures.set_touch_only(false);
        gestures.set_button(gdk::BUTTON_PRIMARY);
        gestures.connect_pressed(glib::clone!(@weak self as obj => move |_, _, _, _| {
            obj.pressed_cb();
        }));
        gestures.connect_released(glib::clone!(@weak self as obj => move |_, _, _, _| {
            obj.released_cb();
        }));
        self.add_controller(gestures);
    }

    fn pressed_cb(&self) {
        if let Some(sm) = self.timer_state_machine() {
            sm.press();
        }
    }

    fn released_cb(&self) {
        if let Some(sm) = self.timer_state_machine() {
            sm.release();
        }
    }

    fn setup_callbacks(&self) {}

    pub(self) fn timer_state_changed_cb(&self, state: TimerState) {
        let imp = self.imp();

        match state {
            TimerState::Idle => {
                self.set_color_normal();
                imp.statistics_box.set_visible(true);
            }
            TimerState::Wait => {
                self.set_color_wait();
                imp.statistics_box.set_visible(true);
            }
            TimerState::Ready => {
                self.set_color_ready();
                self.set_time_label(Duration::ZERO);
                imp.statistics_box.set_visible(false);
            }
            TimerState::Timing { duration } => {
                self.set_color_normal();
                self.set_time_label(duration);
                imp.statistics_box.set_visible(false);
            }
            TimerState::Finished { solve_time, .. } => {
                self.set_color_wait();
                self.set_time_label(solve_time.measured_time());
                imp.statistics_box.set_visible(true);
                if let Some(session) = self.session() {
                    session.add_solve(data::SolveData::new(solve_time, "".to_string()));
                }
            }
        }
    }

    pub(self) fn tick_cb(&self, sm: &data::TimerStateMachine) {
        if let data::TimerState::Timing { duration } = sm.simple_state() {
            self.set_time_label(duration);
        }
    }

    fn set_color_normal(&self) {
        self.remove_css_class("wait");
        self.remove_css_class("ready");
    }

    fn set_color_wait(&self) {
        self.remove_css_class("ready");
        self.add_css_class("wait");
    }

    fn set_color_ready(&self) {
        self.remove_css_class("wait");
        self.add_css_class("ready");
    }

    fn set_time_label(&self, duration: Duration) {
        let imp = self.imp();
        let s = duration.as_secs();
        let m = s / 60;
        let s = s % 60;
        let c = duration.subsec_millis() / 10;

        if m > 0 {
            imp.minutes.set_visible(true);
            imp.colon.set_visible(true);
            imp.minutes.set_label(&format!("{:0>1}", m));
            imp.seconds.set_label(&format!("{:0>2}", s));
        } else {
            imp.minutes.set_visible(false);
            imp.colon.set_visible(false);
            imp.seconds.set_label(&format!("{:0>1}", s));
        }
        imp.centis.set_label(&format!("{:0>2}", c));
    }

    #[template_callback]
    fn breakpoint_apply_cb(&self, _bp: &adw::Breakpoint) {
        self.add_css_class("timer-face-large");
    }

    #[template_callback]
    fn breakpoint_unapply_cb(&self, _bp: &adw::Breakpoint) {
        self.remove_css_class("timer-face-large");
    }
}
