use std::time::Duration;

use crate::data;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::glib;

#[doc(hidden)]
mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/github/manenfu/PrismaTimer/ui/time_label.ui")]
    pub struct TimeLabel {
        #[template_child]
        pub stack: TemplateChild<gtk::Stack>,
        #[template_child]
        pub time_label: TemplateChild<gtk::Box>,
        #[template_child]
        pub plus: TemplateChild<gtk::Label>,
        #[template_child]
        pub minutes: TemplateChild<gtk::Label>,
        #[template_child]
        pub colon: TemplateChild<gtk::Label>,
        #[template_child]
        pub seconds: TemplateChild<gtk::Label>,
        #[template_child]
        pub point: TemplateChild<gtk::Label>,
        #[template_child]
        pub centis: TemplateChild<gtk::Label>,
        #[template_child]
        pub dnf: TemplateChild<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TimeLabel {
        const NAME: &'static str = "PtTimeLabel";
        type Type = super::TimeLabel;
        type ParentType = adw::Bin;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for TimeLabel {}
    impl WidgetImpl for TimeLabel {}
    impl BinImpl for TimeLabel {}
}

glib::wrapper! {
    pub struct TimeLabel(ObjectSubclass<imp::TimeLabel>)
        @extends gtk::Widget, adw::Bin,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl TimeLabel {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn set_duration(&self, duration: Duration) {
        self.set_solve_time(data::SolveTime::new(duration, data::Penalty::Ok));
    }

    pub fn set_solve_time(&self, solve_time: data::SolveTime) {
        let imp = self.imp();

        match solve_time.penalty {
            data::Penalty::Ok => {
                imp.stack.set_visible_child_name("time-label");
                imp.plus.set_visible(false);
            }
            data::Penalty::Plus2 => {
                imp.stack.set_visible_child_name("time-label");
                imp.plus.set_visible(true);
            }
            data::Penalty::Dnf => {
                imp.stack.set_visible_child_name("dnf");
                imp.plus.set_visible(false);
            }
        }

        let recorded_time = solve_time.recorded_time().unwrap_or_default();
        let s = recorded_time.as_secs();
        let m = s / 60;
        let s = s % 60;
        let c = recorded_time.subsec_millis() / 10;

        if m > 0 {
            imp.minutes.set_visible(true);
            imp.colon.set_visible(true);
            imp.minutes.set_label(&format!("{:0>1}", m));
            imp.seconds.set_label(&format!("{:0>2}", s));
        } else {
            imp.minutes.set_visible(false);
            imp.colon.set_visible(false);
            imp.seconds.set_label(&format!("{:0>1}", s));
        }
        imp.centis.set_label(&format!("{:0>2}", c));
    }
}

impl Default for TimeLabel {
    fn default() -> Self {
        Self::new()
    }
}
