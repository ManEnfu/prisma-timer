use adw::subclass::prelude::*;
use gtk::glib;
use gtk::prelude::*;

use crate::data;
use crate::util::TemplateCallbacks;

#[doc(hidden)]
mod imp {
    use std::cell::RefCell;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, glib::Properties)]
    #[template(resource = "/io/github/manenfu/PrismaTimer/ui/session_item_row.ui")]
    #[properties(wrapper_type = super::SessionItemRow)]
    pub struct SessionItemRow {
        #[property(get, set)]
        pub item: RefCell<Option<data::SessionItem>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SessionItemRow {
        const NAME: &'static str = "PtSessionItemRow";
        type Type = super::SessionItemRow;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            TemplateCallbacks::bind_template_callbacks(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for SessionItemRow {}
    impl WidgetImpl for SessionItemRow {}
    impl BinImpl for SessionItemRow {}
}

glib::wrapper! {
    pub struct SessionItemRow(ObjectSubclass<imp::SessionItemRow>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl SessionItemRow {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
}

impl Default for SessionItemRow {
    fn default() -> Self {
        Self::new()
    }
}
