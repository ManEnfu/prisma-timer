use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gdk, gio, glib};

use crate::config;

#[doc(hidden)]
mod imp {
    use once_cell::sync::OnceCell;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/github/manenfu/PrismaTimer/ui/preferences_window.ui")]
    pub struct PreferencesWindow {
        #[template_child]
        pub(super) timer_touch_only_switch: TemplateChild<adw::SwitchRow>,
        #[template_child]
        pub(super) timer_use_countdown_switch: TemplateChild<adw::SwitchRow>,

        #[template_child]
        pub(super) use_system_color_scheme_switch: TemplateChild<adw::SwitchRow>,
        #[template_child]
        pub(super) dark_mode_switch: TemplateChild<adw::SwitchRow>,

        pub(super) settings: OnceCell<gio::Settings>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PreferencesWindow {
        const NAME: &'static str = "PtPreferencesWindow";
        type Type = super::PreferencesWindow;
        type ParentType = adw::PreferencesWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();

            klass.add_binding_action(
                gdk::Key::Escape,
                gdk::ModifierType::empty(),
                "window.close",
                None,
            )
        }

        fn instance_init(obj: &gtk::glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for PreferencesWindow {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();

            obj.setup_settings();

            obj.setup_timer_group();
            obj.setup_appearance_group();
        }
    }

    impl WidgetImpl for PreferencesWindow {}
    impl WindowImpl for PreferencesWindow {}
    impl AdwWindowImpl for PreferencesWindow {}
    impl PreferencesWindowImpl for PreferencesWindow {}
}

glib::wrapper! {
    pub struct PreferencesWindow(ObjectSubclass<imp::PreferencesWindow>)
        @extends gtk::Widget, gtk::Window, adw::Window, adw::PreferencesWindow,
        @implements gtk::Accessible;
}

impl PreferencesWindow {
    pub fn new(window: &impl IsA<gtk::Window>) -> Self {
        glib::Object::builder()
            .property("transient-for", window)
            .build()
    }

    fn setup_settings(&self) {
        let imp = self.imp();

        let settings = gio::Settings::new(config::APP_ID);
        imp.settings
            .set(settings)
            .expect("`settings` should not be set before `setup_settings` is called");
    }

    fn setup_timer_group(&self) {
        let imp = self.imp();
        let settings = self.settings();

        settings
            .bind(
                "timer-touch-only",
                &*imp.timer_touch_only_switch,
                "active",
            )
            .build();

        settings
            .bind(
                "timer-use-countdown",
                &*imp.timer_use_countdown_switch,
                "active",
            )
            .build();
    }

    fn setup_appearance_group(&self) {
        let imp = self.imp();
        let manager = adw::StyleManager::default();
        let settings = self.settings();

        settings
            .bind("dark-mode", &*imp.dark_mode_switch, "active")
            .build();

        if manager.system_supports_color_schemes() {
            imp.use_system_color_scheme_switch.set_sensitive(true);
            imp.use_system_color_scheme_switch
                .bind_property("active", &*imp.dark_mode_switch, "sensitive")
                .sync_create()
                .invert_boolean()
                .build();

            settings
                .bind(
                    "use-system-color-scheme",
                    &*imp.use_system_color_scheme_switch,
                    "active",
                )
                .build();
        } else {
            imp.use_system_color_scheme_switch.set_sensitive(false);
        }
    }

    fn settings(&self) -> &gio::Settings {
        self.imp()
            .settings
            .get()
            .expect("`settings` should be set by `setup_settings` first")
    }
}
