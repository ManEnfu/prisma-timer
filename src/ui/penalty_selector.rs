use adw::subclass::prelude::*;
use gtk::glib;
use gtk::prelude::*;

use crate::data::{Penalty, SessionItem};

#[doc(hidden)]
mod imp {
    use std::cell::{Cell, RefCell};

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, glib::Properties)]
    #[template(resource = "/io/github/manenfu/PrismaTimer/ui/penalty_selector.ui")]
    #[properties(wrapper_type = super::PenaltySelector)]
    pub struct PenaltySelector {
        #[template_child]
        pub ok_button: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub plus2_button: TemplateChild<gtk::ToggleButton>,
        #[template_child]
        pub dnf_button: TemplateChild<gtk::ToggleButton>,

        #[property(get, set = Self::set_penalty, builder(Penalty::default()))]
        pub penalty: Cell<Penalty>,

        #[property(get, set = Self::set_solve, nullable)]
        pub solve: RefCell<Option<SessionItem>>,
        pub solve_binding: RefCell<Option<glib::Binding>>,
    }

    impl PenaltySelector {
        fn set_penalty(&self, v: Penalty) {
            self.penalty.set(v);
            match v {
                Penalty::Ok => self.ok_button.set_active(true),
                Penalty::Plus2 => self.plus2_button.set_active(true),
                Penalty::Dnf => self.dnf_button.set_active(true),
            }
        }

        fn set_solve(&self, v: Option<SessionItem>) {
            let obj = self.obj();
            if let Some(binding) = self.solve_binding.take() {
                binding.unbind();
            }

            if let Some(solve) = &v {
                self.solve_binding.replace(Some(
                    solve
                        .bind_property("penalty", &*obj, "penalty")
                        .sync_create()
                        .bidirectional()
                        .build(),
                ));
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PenaltySelector {
        const NAME: &'static str = "PtPenaltySelector";
        type Type = super::PenaltySelector;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for PenaltySelector {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.set_penalty(Penalty::Ok);
        }
    }
    impl WidgetImpl for PenaltySelector {}
    impl BinImpl for PenaltySelector {}
}

glib::wrapper! {
    pub struct PenaltySelector(ObjectSubclass<imp::PenaltySelector>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

#[gtk::template_callbacks]
impl PenaltySelector {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    #[template_callback]
    fn ok_button_clicked_cb(&self, _: &gtk::ToggleButton) {
        self.set_penalty(Penalty::Ok)
    }

    #[template_callback]
    fn plus2_button_clicked_cb(&self, _: &gtk::ToggleButton) {
        self.set_penalty(Penalty::Plus2)
    }

    #[template_callback]
    fn dnf_button_clicked_cb(&self, _: &gtk::ToggleButton) {
        self.set_penalty(Penalty::Dnf)
    }
}

impl Default for PenaltySelector {
    fn default() -> Self {
        Self::new()
    }
}
