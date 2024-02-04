use std::time::Duration;

use crate::data;
use crate::prelude::*;
use crate::subclass::prelude::*;
use crate::ui;
use gtk::{gdk, glib};

#[doc(hidden)]
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
        pub time_label: TemplateChild<ui::TimeLabel>,

        #[template_child]
        pub penalty_selector: TemplateChild<ui::PenaltySelector>,

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

        #[property(get, set = Self::set_last_solve, nullable)]
        pub last_solve: RefCell<Option<data::SessionItem>>,
        last_solve_handlers: RefCell<Vec<glib::SignalHandlerId>>,
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
                        obj.timer_state_changed_cb(sm);
                    }),
                ));
            }
            self.timer_state_machine.replace(v);
        }

        fn set_last_solve(&self, v: Option<data::SessionItem>) {
            let obj = self.obj();
            let mut handlers = self.last_solve_handlers.borrow_mut();

            if let Some(solve) = self.last_solve.take() {
                for handler in handlers.drain(..) {
                    solve.disconnect(handler);
                }
            }

            if let Some(solve) = &v {
                handlers.push(solve.connect_notify_local(
                    Some("solve-time-string"),
                    glib::clone!(@weak obj => move |solve, _| {
                        obj.last_solve_time_changed_cb(solve);
                    }),
                ))
            }
            self.last_solve.replace(v);
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

            self.time_label.set_duration(Duration::ZERO);
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
        gestures.set_propagation_phase(gtk::PropagationPhase::Capture);
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

    #[template_callback]
    fn notify_has_focus_cb(&self, _pspec: &glib::ParamSpec, _s: &Self) {
        if !self.has_focus() {
            self.released_cb();
        }
    }

    fn setup_callbacks(&self) {}

    pub(self) fn timer_state_changed_cb(&self, sm: &data::TimerStateMachine) {
        let imp = self.imp();

        let is_running = sm.is_running();
        let is_finished = sm.is_finished();
        let content = sm.content();

        imp.statistics_box.set_visible(!is_running);
        imp.penalty_selector.set_visible(!is_running);

        self.set_content(&content);

        if is_finished {
            if let Some(data::TimerContentValue::SolveTime(st)) = content.value {
                self.submit_solve(data::SolveData::new(st, "".to_string()));
            }
        }
    }

    fn set_content(&self, content: &data::TimerContent) {
        if let Some(ref value) = content.value {
            self.set_content_value(value);
        }
        self.set_content_color(content.color);
    }

    fn set_content_value(&self, value: &data::TimerContentValue) {
        let imp = self.imp();
        match value {
            data::TimerContentValue::SolveTime(solve_time) => {
                imp.time_label.set_solve_time(*solve_time)
            }
            data::TimerContentValue::Int(_) => todo!(),
            data::TimerContentValue::String(_) => todo!(),
        }
    }

    fn set_content_color(&self, color: data::TimerContentColor) {
        match color {
            data::TimerContentColor::Neutral => {
                self.remove_css_class("wait");
                self.remove_css_class("ready");
            }
            data::TimerContentColor::Destructive => {
                self.remove_css_class("ready");
                self.add_css_class("wait");
            }
            data::TimerContentColor::Success => {
                self.remove_css_class("wait");
                self.add_css_class("ready");
            }
        }
    }

    fn submit_solve(&self, solve: data::SolveData) {
        let imp = self.imp();
        if let Some(session) = self.session() {
            let session_item = session.add_solve(solve);
            imp.penalty_selector.set_solve(Some(session_item.clone()));
            self.set_last_solve(Some(session_item));
        }
    }

    fn last_solve_time_changed_cb(&self, solve: &data::SessionItem) {
        let imp = self.imp();
        imp.time_label.set_solve_time(solve.time());
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
