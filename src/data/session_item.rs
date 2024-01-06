use std::time::SystemTime;

use gtk::glib;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use crate::data::{Penalty, SolveData, SolveTime, SolvesSeq};

#[allow(clippy::enum_variant_names)]
mod imp {
    use std::cell::{Cell, RefCell};

    use super::*;

    #[derive(Debug, Default, glib::Properties)]
    #[properties(wrapper_type = super::SessionItem)]
    pub struct SessionItem {
        #[property(name = "recorded-time", type = String, get = Self::get_recorded_time_string)]
        pub solve: RefCell<Option<SolveData>>,
        #[property(name = "mo3-time", type = String, get = Self::get_mo3_string)]
        pub mo3: Cell<Option<SolveTime>>,
        #[property(name = "ao5-time", type = String, get = Self::get_ao5_string)]
        pub ao5: Cell<Option<SolveTime>>,
        #[property(name = "ao12-time", type = String, get = Self::get_ao12_string)]
        pub ao12: Cell<Option<SolveTime>>,
    }

    impl SessionItem {
        fn get_recorded_time_string(&self) -> String {
            self.solve.borrow().as_ref().unwrap().time.to_string()
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
    pub struct SessionItem(ObjectSubclass<imp::SessionItem>);
}

impl SessionItem {
    pub fn new(data: SolveData) -> Self {
        let obj = glib::Object::builder::<Self>().build();
        obj.imp().solve.replace(Some(data));
        obj
    }

    pub fn time(&self) -> SolveTime {
        self.imp().solve.borrow().as_ref().unwrap().time
    }

    pub fn mo3(&self) -> Option<SolveTime> {
        self.imp().mo3.get()
    }

    pub fn ao5(&self) -> Option<SolveTime> {
        self.imp().ao5.get()
    }

    pub fn ao12(&self) -> Option<SolveTime> {
        self.imp().ao12.get()
    }

    pub fn set_mo3(&self, v: Option<SolveTime>) {
        self.imp().mo3.set(v);
        self.notify_mo3_time();
    }

    pub fn set_ao5(&self, v: Option<SolveTime>) {
        self.imp().ao5.set(v);
        self.notify_ao5_time();
    }

    pub fn set_ao12(&self, v: Option<SolveTime>) {
        self.imp().ao12.set(v);
        self.notify_ao12_time();
    }

    pub fn set_penalty(&self, penalty: Option<Penalty>) {
        self.imp().solve.borrow_mut().as_mut().unwrap().time.penalty = penalty;
        self.notify_recorded_time();
    }

    pub fn timestamp(&self) -> SystemTime {
        self.imp().solve.borrow().as_ref().unwrap().timestamp
    }
}

impl SolvesSeq for &[SessionItem] {
    fn mean_of_n(&self) -> Option<SolveTime> {
        let len = self.len() as u32;
        if len == 0 {
            return None;
        }

        let sum: SolveTime = self.iter().map(|item| item.time()).sum();
        Some(sum / len)
    }

    fn average_of_n(&self) -> Option<SolveTime> {
        let len = self.len() as u32;
        if len < 3 {
            return None;
        }

        let it = self.iter().map(|se| se.time()).enumerate();

        let (imax, _max) = it.clone().max_by_key(|&(_, st)| st)?;
        let (imin, _min) = it.clone().min_by_key(|&(_, st)| st)?;
        let sum = it.fold(SolveTime::default(), |acc, (i, st)| {
            if i != imax && i != imin {
                acc + st
            } else {
                acc
            }
        });
        Some(sum / (len - 2))
    }
}
