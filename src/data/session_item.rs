use std::time::SystemTime;

use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use crate::data::{Penalty, SolveData, SolveTime};

const EXPECT_INITIALIZED: &str = "`SolveData` haven't yet initialized in `SessionItem`";

#[allow(clippy::enum_variant_names)]
#[doc(hidden)]
mod imp {
    use std::cell::{Cell, RefCell};

    use super::*;

    #[derive(Debug, Default, glib::Properties)]
    #[properties(wrapper_type = super::SessionItem)]
    pub struct SessionItem {
        #[property(name = "solve-time-string", type = String, get = Self::get_recorded_time_string)]
        #[property(name = "penalty", type = Penalty, get = Self::get_penalty, set = Self::set_penalty, builder(Penalty::default()))]
        #[property(name = "timestamp-string", type = String, get = Self::get_timestamp_string)]
        pub solve: RefCell<Option<SolveData>>,
        #[property(name = "mo3-string", type = String, get = Self::get_mo3_string)]
        pub mo3: Cell<Option<SolveTime>>,
        #[property(name = "ao5-string", type = String, get = Self::get_ao5_string)]
        pub ao5: Cell<Option<SolveTime>>,
        #[property(name = "ao12-string", type = String, get = Self::get_ao12_string)]
        pub ao12: Cell<Option<SolveTime>>,
    }

    impl SessionItem {
        fn get_recorded_time_string(&self) -> String {
            self.solve
                .borrow()
                .as_ref()
                .expect(EXPECT_INITIALIZED)
                .time
                .to_string()
        }

        fn get_mo3_string(&self) -> String {
            self.mo3.get().map_or(String::default(), |t| t.to_string())
        }

        fn get_ao5_string(&self) -> String {
            self.ao5.get().map_or(String::default(), |t| t.to_string())
        }

        fn get_ao12_string(&self) -> String {
            self.ao12.get().map_or(String::default(), |t| t.to_string())
        }

        fn get_penalty(&self) -> Penalty {
            self.solve
                .borrow()
                .as_ref()
                .expect(EXPECT_INITIALIZED)
                .time
                .penalty
        }

        fn set_penalty(&self, v: Penalty) {
            self.solve
                .borrow_mut()
                .as_mut()
                .expect(EXPECT_INITIALIZED)
                .time
                .penalty = v;
            self.obj().notify_solve_time_string();
        }

        fn get_timestamp_string(&self) -> String {
            let timestamp = self
                .solve
                .borrow()
                .as_ref()
                .expect(EXPECT_INITIALIZED)
                .timestamp;
            let dt = chrono::DateTime::<chrono::Local>::from(timestamp);
            dt.format("%Y-%m-%d %H:%M:%S").to_string()
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SessionItem {
        const NAME: &'static str = "PtSessionItem";
        type Type = super::SessionItem;
    }

    #[glib::derived_properties]
    impl ObjectImpl for SessionItem {}
}

glib::wrapper! {
    /// An item in a `Session`.
    pub struct SessionItem(ObjectSubclass<imp::SessionItem>);
}

impl SessionItem {
    /// Creates a new `SessionItem` from a `SolveData`
    pub fn new(data: SolveData) -> Self {
        let obj = glib::Object::builder::<Self>().build();
        obj.imp().solve.replace(Some(data));
        obj
    }

    /// Gets the solve time of this item.
    pub fn time(&self) -> SolveTime {
        self.imp()
            .solve
            .borrow()
            .as_ref()
            .expect(EXPECT_INITIALIZED)
            .time
    }

    /// Gets the mean of 3 of this item.
    pub fn mo3(&self) -> Option<SolveTime> {
        self.imp().mo3.get()
    }

    /// Gets the average of 5 of this item.
    pub fn ao5(&self) -> Option<SolveTime> {
        self.imp().ao5.get()
    }

    /// Gets the average of 12 of this item.
    pub fn ao12(&self) -> Option<SolveTime> {
        self.imp().ao12.get()
    }

    /// Sets the mean of 3 of this item.
    pub(crate) fn set_mo3(&self, v: Option<SolveTime>) {
        self.imp().mo3.set(v);
        self.notify_mo3_string();
    }

    /// Sets the average of 5 of this item.
    pub(crate) fn set_ao5(&self, v: Option<SolveTime>) {
        self.imp().ao5.set(v);
        self.notify_ao5_string();
    }

    /// Sets the average of 12 of this item.
    pub(crate) fn set_ao12(&self, v: Option<SolveTime>) {
        self.imp().ao12.set(v);
        self.notify_ao12_string();
    }

    /// Gets the timestamp of this item.
    pub(crate) fn timestamp(&self) -> SystemTime {
        self.imp()
            .solve
            .borrow()
            .as_ref()
            .expect(EXPECT_INITIALIZED)
            .timestamp
    }
}

impl From<&SessionItem> for SolveTime {
    fn from(value: &SessionItem) -> Self {
        value.time()
    }
}
