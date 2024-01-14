use crate::data;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib;

#[doc(hidden)]
mod imp {
    use once_cell::sync::OnceCell;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, glib::Properties)]
    #[template(resource = "/io/github/manenfu/PrismaTimer/ui/solve_dialog.ui")]
    #[properties(wrapper_type = super::SolveDialog)]
    pub struct SolveDialog {
        #[property(get, construct_only)]
        pub session: OnceCell<data::Session>,
        #[property(get, construct_only)]
        pub index: OnceCell<u32>,
        #[property(get, construct_only)]
        pub solve: OnceCell<Option<data::SessionItem>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SolveDialog {
        const NAME: &'static str = "PtSolveDialog";
        type Type = super::SolveDialog;
        type ParentType = adw::Window;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for SolveDialog {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            obj.setup();
        }
    }

    impl WidgetImpl for SolveDialog {}
    impl WindowImpl for SolveDialog {}
    impl AdwWindowImpl for SolveDialog {}
}

glib::wrapper! {
    pub struct SolveDialog(ObjectSubclass<imp::SolveDialog>)
        @extends gtk::Widget, gtk::Window, adw::Window;
}

#[gtk::template_callbacks]
impl SolveDialog {
    pub fn new(session: data::Session, index: u32) -> Self {
        let solve = session.get(index as usize);

        glib::Object::builder()
            .property("session", session)
            .property("index", index)
            .property("solve", solve)
            .build()
    }

    fn setup(&self) {
        self.set_title(Some(&Self::create_window_title(self.index())));
    }

    #[template_callback(function)]
    fn create_window_title(index: u32) -> String {
        format!("Solve {}", index + 1)
    }
}
