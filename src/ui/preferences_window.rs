use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gdk, glib};

#[doc(hidden)]
mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/github/manenfu/PrismaTimer/ui/preferences_window.ui")]
    pub struct PreferencesWindow {}

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

    impl ObjectImpl for PreferencesWindow {}
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
}
