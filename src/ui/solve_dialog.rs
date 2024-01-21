use crate::data;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib;

#[doc(hidden)]
mod imp {
    use std::cell::RefCell;

    use once_cell::sync::OnceCell;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, glib::Properties)]
    #[template(resource = "/io/github/manenfu/PrismaTimer/ui/solve_dialog.ui")]
    #[properties(wrapper_type = super::SolveDialog)]
    pub struct SolveDialog {
        #[template_child]
        pub average_group: TemplateChild<adw::PreferencesGroup>,
        #[template_child]
        pub ao5_expander_row: TemplateChild<adw::ExpanderRow>,
        #[template_child]
        pub ao12_expander_row: TemplateChild<adw::ExpanderRow>,

        #[property(get, construct_only)]
        pub session: OnceCell<data::Session>,
        #[property(get, construct_only)]
        pub index: OnceCell<u32>,
        #[property(get, construct_only)]
        pub solve: RefCell<Option<data::SessionItem>>,
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
        let imp = self.imp();
        let session = self.session();
        let index = self.index();
        self.set_title(Some(&Self::create_window_title(index)));
        imp.average_group.set_visible(index >= 4);
        Self::setup_average_expander(&imp.ao5_expander_row, session.get_slice(index as usize, 5));
        Self::setup_average_expander(
            &imp.ao12_expander_row,
            session.get_slice(index as usize, 12),
        );
    }

    fn setup_average_expander(
        expander_row: &adw::ExpanderRow,
        solves: Option<Vec<data::SessionItem>>,
    ) {
        if let Some(_solves) = solves {
            expander_row.set_visible(true);
        } else {
            expander_row.set_visible(false);
        }
    }

    #[template_callback(function)]
    fn create_window_title(index: u32) -> String {
        format!("Solve {}", index + 1)
    }

    #[template_callback]
    fn remove_button_clicked_cb(&self, _button: &gtk::Button) {
        self.confirm_remove_dialog();
    }

    fn confirm_remove_dialog(&self) {
        let builder = gtk::Builder::from_resource(
            "/io/github/manenfu/PrismaTimer/ui/confirm_remove_dialog.ui",
        );
        let dialog = builder
            .object::<adw::MessageDialog>("dialog")
            .expect("Expected dialog");
        dialog.set_transient_for(Some(self));
        dialog.connect_response(
            Some("remove"),
            glib::clone!(@weak self as obj => move |dialog, _| {
                dialog.close();
                dialog.set_transient_for(Option::<&Self>::None);
                obj.remove_solve();
            }),
        );
        dialog.present();
    }

    fn remove_solve(&self) {
        if let Some(solve) = self.solve() {
            self.session().remove_solve_by_object(&solve);
        }
        self.close();
    }
}
