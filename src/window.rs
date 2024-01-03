use crate::{data, ui};
use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gdk, gio, glib};

mod imp {
    use std::cell::RefCell;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, glib::Properties)]
    #[template(resource = "/io/github/manenfu/PrismaTimer/ui/window.ui")]
    #[properties(wrapper_type = super::PrismaTimerWindow)]
    pub struct PrismaTimerWindow {
        #[template_child]
        pub sidebar_header_bar: TemplateChild<adw::HeaderBar>,
        #[template_child]
        pub content_header_bar: TemplateChild<adw::HeaderBar>,
        #[template_child]
        pub timer_face: TemplateChild<ui::TimerFace>,

        #[property(get, set)]
        pub timer_state_machine: RefCell<Option<data::TimerStateMachine>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PrismaTimerWindow {
        const NAME: &'static str = "PrismaTimerWindow";
        type Type = super::PrismaTimerWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for PrismaTimerWindow {
        fn properties() -> &'static [glib::ParamSpec] {
            Self::derived_properties()
        }

        fn set_property(&self, id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {
            self.derived_set_property(id, value, pspec)
        }

        fn property(&self, id: usize, pspec: &glib::ParamSpec) -> glib::Value {
            self.derived_property(id, pspec)
        }

        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            obj.set_timer_state_machine(data::TimerStateMachine::new());

            obj.setup_gactions();
            obj.setup_event_controllers();
        }
    }

    impl WidgetImpl for PrismaTimerWindow {}
    impl WindowImpl for PrismaTimerWindow {}
    impl ApplicationWindowImpl for PrismaTimerWindow {}
    impl AdwApplicationWindowImpl for PrismaTimerWindow {}
}

glib::wrapper! {
    pub struct PrismaTimerWindow(ObjectSubclass<imp::PrismaTimerWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl PrismaTimerWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }

    fn setup_gactions(&self) {
        let shortcuts_window =
            gtk::Builder::from_resource("/io/github/manenfu/PrismaTimer/ui/shortcuts_window.ui")
                .object::<gtk::ShortcutsWindow>("shortcuts_window")
                .expect("Error building shortcuts window.");
        self.set_help_overlay(Some(&shortcuts_window));
    }

    fn setup_event_controllers(&self) {
        // Focus on timer face if user clicks on anywhere on window that has no
        // interactable widgets.
        let gestures = gtk::GestureClick::new();
        gestures.set_touch_only(false);
        gestures.set_button(gdk::BUTTON_PRIMARY);
        gestures.set_propagation_phase(gtk::PropagationPhase::Capture);
        gestures.connect_pressed(glib::clone!(@weak self as obj => move |_, _, _, _| {
            obj.imp().timer_face.grab_focus();
        }));
        self.add_controller(gestures);
    }
}
