use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gdk, glib};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/github/manenfu/PrismaTimer/ui/timer_face.ui")]
    pub struct TimerFace {
        #[template_child]
        pub minutes: TemplateChild<gtk::Label>,
        #[template_child]
        pub colon: TemplateChild<gtk::Label>,
        #[template_child]
        pub seconds: TemplateChild<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TimerFace {
        const NAME: &'static str = "PtTimerFace";
        type Type = super::TimerFace;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for TimerFace {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            obj.add_css_class("timer-face");

            obj.setup_event_controllers();
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
        log::debug!("PtTimerFace pressed");
        self.add_css_class("wait");
    }

    fn released_cb(&self) {
        log::debug!("PtTimerFace released");
        self.remove_css_class("wait");
    }
}
