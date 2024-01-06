use crate::data::{SessionItem, SolveData, SolveTime, SolvesSeq};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

mod imp {
    use std::cell::RefCell;

    use gtk::glib::subclass::{Signal, SignalType};
    use once_cell::sync::Lazy;

    use super::*;

    #[derive(Debug, Default)]
    pub struct Session {
        pub list: RefCell<Vec<SessionItem>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Session {
        const NAME: &'static str = "PtSession";
        type Type = super::Session;
        type Interfaces = (gio::ListModel,);
    }

    impl ObjectImpl for Session {
        fn signals() -> &'static [Signal] {
            static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
                vec![Signal::builder("solve-added")
                    .param_types(Vec::<SignalType>::new())
                    .build()]
            });
            SIGNALS.as_ref()
        }
    }

    impl ListModelImpl for Session {
        fn item_type(&self) -> glib::Type {
            SessionItem::static_type()
        }

        fn n_items(&self) -> u32 {
            self.list.borrow().len() as u32
        }

        fn item(&self, position: u32) -> Option<glib::Object> {
            self.list
                .borrow()
                .get(position as usize)
                .cloned()
                .and_upcast()
        }
    }
}

glib::wrapper! {
    /// A solving session.
    pub struct Session(ObjectSubclass<imp::Session>)
        @implements gio::ListModel;
}

impl Session {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn last_solve(&self) -> Option<SessionItem> {
        self.imp().list.borrow().last().cloned()
    }

    pub fn get(&self, index: usize) -> Option<SessionItem> {
        self.imp().list.borrow().get(index).cloned()
    }

    pub fn add_solve(&self, solve: SolveData) {
        let item = SessionItem::new(solve);
        self.imp().list.borrow_mut().push(item);
        let index = self.n_items() - 1;
        self.items_changed(index, 0, 1);
        self.solve_updated(index as usize);
        self.emit_by_name::<()>("solve-added", &[]);
    }

    pub fn best_solve(&self) -> Option<SessionItem> {
        self.imp()
            .list
            .borrow()
            .iter()
            .min_by_key(|item| item.time())
            .cloned()
    }

    pub fn best_mo3(&self) -> Option<SolveTime> {
        self.imp()
            .list
            .borrow()
            .iter()
            .min_by_key(|item| item.mo3().unwrap_or(SolveTime::DNF))
            .and_then(SessionItem::mo3)
    }

    pub fn best_ao5(&self) -> Option<SolveTime> {
        self.imp()
            .list
            .borrow()
            .iter()
            .min_by_key(|item| item.ao5().unwrap_or(SolveTime::DNF))
            .and_then(SessionItem::ao5)
    }

    pub fn best_ao12(&self) -> Option<SolveTime> {
        self.imp()
            .list
            .borrow()
            .iter()
            .min_by_key(|item| item.ao12().unwrap_or(SolveTime::DNF))
            .and_then(SessionItem::ao12)
    }

    fn compute_mo3(&self, index: usize) -> Option<SolveTime> {
        let list = self.imp().list.borrow();
        if list.len() >= 3 {
            list.get(index - 2..index + 1)
                .and_then(|solves| solves.mean_of_n())
        } else {
            None
        }
    }

    fn compute_ao5(&self, index: usize) -> Option<SolveTime> {
        let list = self.imp().list.borrow();
        if list.len() >= 5 {
            list.get(index - 4..index + 1)
                .and_then(|solves| solves.average_of_n())
        } else {
            None
        }
    }

    fn compute_ao12(&self, index: usize) -> Option<SolveTime> {
        let list = self.imp().list.borrow();
        if list.len() >= 12 {
            list.get(index - 11..index + 1)
                .and_then(|solves| solves.average_of_n())
        } else {
            None
        }
    }

    fn update_mo3(&self, index: usize) {
        let mo3 = self.compute_mo3(index);
        if let Some(item) = self.imp().list.borrow().get(index) {
            item.set_mo3(mo3);
        }
    }

    fn update_ao5(&self, index: usize) {
        let ao5 = self.compute_ao5(index);
        if let Some(item) = self.imp().list.borrow().get(index) {
            item.set_ao5(ao5);
        }
    }

    fn update_ao12(&self, index: usize) {
        let ao12 = self.compute_ao12(index);
        if let Some(item) = self.imp().list.borrow().get(index) {
            item.set_ao12(ao12);
        }
    }

    pub fn solve_updated(&self, index: usize) {
        let len = self.n_items() as usize;

        for i in index..len.min(index + 3) {
            self.update_mo3(i);
        }
        for i in index..len.min(index + 5) {
            self.update_ao5(i);
        }
        for i in index..len.min(index + 12) {
            self.update_ao12(i);
        }
        let n_changed = 12.min(len - index);
        self.items_changed(index as u32, n_changed as u32, n_changed as u32);
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}
