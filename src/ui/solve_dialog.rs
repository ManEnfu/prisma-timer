use crate::data::{self, SolveStatistic};
use crate::ui;
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
        pub ao5_report_area: TemplateChild<ui::TextAreaRow>,
        #[template_child]
        pub ao12_expander_row: TemplateChild<adw::ExpanderRow>,
        #[template_child]
        pub ao12_report_area: TemplateChild<ui::TextAreaRow>,

        #[property(get, construct_only)]
        pub session: OnceCell<data::Session>,
        #[property(get, construct_only)]
        pub index: OnceCell<u32>,
        #[property(get, construct_only)]
        pub solve: RefCell<Option<data::SessionItem>>,

        pub last_5_solves: RefCell<Option<Vec<data::SessionItem>>>,
        pub last_12_solves: RefCell<Option<Vec<data::SessionItem>>>,
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
        let solve = session.get_solve(index as usize);

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

        imp.last_5_solves
            .replace(session.get_solve_slice(index as usize, 5));
        imp.last_12_solves
            .replace(session.get_solve_slice(index as usize, 12));
        self.update_ao5_expander_row();
        self.update_ao12_expander_row();

        if let Some(solve) = self.solve() {
            solve.connect_notify_local(
                Some("solve-time-string"),
                glib::clone!(@weak self as obj => move |_, _| {
                    obj.update_ao5_expander_row();
                    obj.update_ao12_expander_row();
                }),
            );
        }
    }

    fn update_ao5_expander_row(&self) {
        let imp = self.imp();
        Self::update_average_expander(
            &imp.ao5_expander_row,
            &imp.ao5_report_area,
            imp.last_5_solves.borrow().as_ref().map(Vec::as_slice),
        );
    }

    fn update_ao12_expander_row(&self) {
        let imp = self.imp();
        Self::update_average_expander(
            &imp.ao12_expander_row,
            &imp.ao12_report_area,
            imp.last_12_solves.borrow().as_ref().map(Vec::as_slice),
        );
    }

    fn update_average_expander(
        expander_row: &adw::ExpanderRow,
        text_area_row: &ui::TextAreaRow,
        solves: Option<&[data::SessionItem]>,
    ) {
        if let Some(solves) = solves {
            expander_row.set_visible(true);
            text_area_row
                .buffer()
                .set_text(&generate_average_of_n_report(solves).unwrap_or_default());
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

fn generate_average_of_n_report(solves: &[data::SessionItem]) -> Option<String> {
    let ibest = solves.best_solve_index()?;
    let iworst = solves.worst_solve_index()?;
    let header = [
        format!("avg of {}: {}\n\n", solves.len(), solves.average_of_n()?),
        "Time list:\n".to_string(),
    ]
    .into_iter();
    let time_list = solves
        .iter()
        .map(Into::<data::SolveTime>::into)
        .enumerate()
        .map(|(i, time)| {
            if i == ibest || i == iworst {
                format!("{}. ({})\n", i + 1, time)
            } else {
                format!("{}. {}\n", i + 1, time)
            }
        });
    Some(String::from_iter(header.chain(time_list)))
}
