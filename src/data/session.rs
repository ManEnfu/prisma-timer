use crate::data::{SessionItem, SolveData, SolveTime, SolvesSeq};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

#[allow(clippy::enum_variant_names)]
#[doc(hidden)]
mod imp {
    use std::cell::RefCell;

    use gtk::glib::subclass::{Signal, SignalType};
    use once_cell::sync::Lazy;

    use super::*;

    #[derive(Debug, Default, glib::Properties)]
    #[properties(wrapper_type = super::Session)]
    pub struct Session {
        #[property(name = "last-solve-string", type = String, get = Self::get_last_solve_string)]
        #[property(name = "last-mo3-string", type = String, get = Self::get_last_mo3_string)]
        #[property(name = "last-ao5-string", type = String, get = Self::get_last_ao5_string)]
        #[property(name = "last-ao12-string", type = String, get = Self::get_last_ao12_string)]
        #[property(name = "best-solve-string", type = String, get = Self::get_best_solve_string)]
        #[property(name = "best-mo3-string", type = String, get = Self::get_best_mo3_string)]
        #[property(name = "best-ao5-string", type = String, get = Self::get_best_ao5_string)]
        #[property(name = "best-ao12-string", type = String, get = Self::get_best_ao12_string)]
        pub list: RefCell<Vec<SessionItem>>,
    }

    impl Session {
        fn get_last_solve_string(&self) -> String {
            self.list
                .borrow()
                .last()
                .map(SessionItem::time)
                .map_or(String::default(), |t| t.to_string())
        }

        fn get_last_mo3_string(&self) -> String {
            self.list
                .borrow()
                .last()
                .and_then(SessionItem::mo3)
                .map_or(String::default(), |t| t.to_string())
        }

        fn get_last_ao5_string(&self) -> String {
            self.list
                .borrow()
                .last()
                .and_then(SessionItem::ao5)
                .map_or(String::default(), |t| t.to_string())
        }

        fn get_last_ao12_string(&self) -> String {
            self.list
                .borrow()
                .last()
                .and_then(SessionItem::ao12)
                .map_or(String::default(), |t| t.to_string())
        }

        fn get_best_solve_string(&self) -> String {
            self.list
                .borrow()
                .iter()
                .map(SessionItem::time)
                .min()
                .map_or(String::default(), |t| t.to_string())
        }

        fn get_best_mo3_string(&self) -> String {
            self.list
                .borrow()
                .iter()
                .min_by_key(|item| item.mo3().unwrap_or(SolveTime::DNF))
                .and_then(SessionItem::mo3)
                .map_or(String::default(), |t| t.to_string())
        }

        fn get_best_ao5_string(&self) -> String {
            self.list
                .borrow()
                .iter()
                .min_by_key(|item| item.ao5().unwrap_or(SolveTime::DNF))
                .and_then(SessionItem::ao5)
                .map_or(String::default(), |t| t.to_string())
        }

        fn get_best_ao12_string(&self) -> String {
            self.list
                .borrow()
                .iter()
                .min_by_key(|item| item.ao12().unwrap_or(SolveTime::DNF))
                .and_then(SessionItem::ao12)
                .map_or(String::default(), |t| t.to_string())
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Session {
        const NAME: &'static str = "PtSession";
        type Type = super::Session;
        type Interfaces = (gio::ListModel,);
    }

    #[glib::derived_properties]
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
    /// Creates a new session.
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    /// Gets the last solve of this session.
    pub fn last_solve(&self) -> Option<SessionItem> {
        self.imp().list.borrow().last().cloned()
    }

    /// Gets the nth solve of this session.
    pub fn get(&self, index: usize) -> Option<SessionItem> {
        self.imp().list.borrow().get(index).cloned()
    }

    /// Adds a solve to this session.
    pub fn add_solve(&self, solve: SolveData) {
        let item = SessionItem::new(solve);
        self.imp().list.borrow_mut().push(item);
        let index = self.n_items() - 1;
        self.items_changed(index, 0, 1);
        self.solve_updated(index as usize);
        self.emit_by_name::<()>("solve-added", &[]);
    }

    /// Gets the best solve item of this session.
    pub fn best_solve(&self) -> Option<SessionItem> {
        self.imp()
            .list
            .borrow()
            .iter()
            .min_by_key(|item| item.time())
            .cloned()
    }

    /// Gets the best mean of 3 time of this session.
    pub fn best_mo3(&self) -> Option<SolveTime> {
        self.imp()
            .list
            .borrow()
            .iter()
            .min_by_key(|item| item.mo3().unwrap_or(SolveTime::DNF))
            .and_then(SessionItem::mo3)
    }

    /// Gets the best average of 5 time of this session.
    pub fn best_ao5(&self) -> Option<SolveTime> {
        self.imp()
            .list
            .borrow()
            .iter()
            .min_by_key(|item| item.ao5().unwrap_or(SolveTime::DNF))
            .and_then(SessionItem::ao5)
    }

    /// Gets the best average of 12 time of this session.
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
        if index + 1 >= 3 {
            list.get(index - 2..index + 1)
                .and_then(|solves| solves.mean_of_n())
        } else {
            None
        }
    }

    fn compute_ao5(&self, index: usize) -> Option<SolveTime> {
        let list = self.imp().list.borrow();
        if index + 1 >= 5 {
            list.get(index - 4..index + 1)
                .and_then(|solves| solves.average_of_n())
        } else {
            None
        }
    }

    fn compute_ao12(&self, index: usize) -> Option<SolveTime> {
        let list = self.imp().list.borrow();
        if index + 1 >= 12 {
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

    /// Notify updates of an item in this index.
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
        self.notify_last_solve_string();
        self.notify_last_mo3_string();
        self.notify_last_ao5_string();
        self.notify_last_ao12_string();
        self.notify_best_solve_string();
        self.notify_best_mo3_string();
        self.notify_best_ao5_string();
        self.notify_last_ao12_string();
    }

    /// Notify updates of an `SessionItem` object in this session.
    pub fn solve_updated_by_object(&self, obj: &SessionItem) {
        if let Some(index) = self.imp().list.borrow().iter().position(|item| item == obj) {
            self.solve_updated(index);
        } else {
            log::warn!("PtSessionItem object is not in Session");
        }
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, SystemTime};

    use crate::data::Penalty;

    use super::*;

    fn add_dummy_solve(session: &Session, solve_time: SolveTime) {
        session.add_solve(SolveData {
            time: solve_time,
            timestamp: SystemTime::now(),
            scramble: String::default(),
        })
    }

    #[test]
    fn simulate_session() {
        let session = Session::new();
        add_dummy_solve(
            &session,
            SolveTime::new(Duration::from_millis(13_440), None),
        );
        add_dummy_solve(
            &session,
            SolveTime::new(Duration::from_millis(14_320), None),
        );
        add_dummy_solve(
            &session,
            SolveTime::new(Duration::from_millis(15_900), None),
        );
        add_dummy_solve(
            &session,
            SolveTime::new(Duration::from_millis(12_530), None),
        );
        add_dummy_solve(
            &session,
            SolveTime::new(Duration::from_millis(13_080), None),
        );
        add_dummy_solve(
            &session,
            SolveTime::new(Duration::from_millis(14_650), None),
        );
        add_dummy_solve(
            &session,
            SolveTime::new(Duration::from_millis(13_540), None),
        );
        add_dummy_solve(
            &session,
            SolveTime::new(Duration::from_millis(12_940), None),
        );
        add_dummy_solve(
            &session,
            SolveTime::new(Duration::from_millis(12_110), None),
        );
        add_dummy_solve(
            &session,
            SolveTime::new(Duration::from_millis(13_890), None),
        );
        add_dummy_solve(
            &session,
            SolveTime::new(Duration::from_millis(14_330), None),
        );
        add_dummy_solve(
            &session,
            SolveTime::new(Duration::from_millis(15_020), None),
        );

        let last_solve = session.last_solve().unwrap();
        let last_time = last_solve.time();
        assert!(last_time.eq_aprrox(&SolveTime::new(Duration::from_millis(15_020), None), 10));
        let last_mo3 = last_solve.mo3().unwrap();
        assert!(last_mo3.eq_aprrox(&SolveTime::new(Duration::from_millis(14_410), None), 10));
        let last_ao5 = last_solve.ao5().unwrap();
        assert!(last_ao5.eq_aprrox(&SolveTime::new(Duration::from_millis(13_720), None), 10));
        let last_ao12 = last_solve.ao12().unwrap();
        assert!(last_ao12.eq_aprrox(&SolveTime::new(Duration::from_millis(13_770), None), 10));
        let best_time = session.best_solve().unwrap().time();
        assert!(best_time.eq_aprrox(&SolveTime::new(Duration::from_millis(12_110), None), 10));
        let best_mo3 = session.best_mo3().unwrap();
        assert!(best_mo3.eq_aprrox(&SolveTime::new(Duration::from_millis(12_860), None), 10));
        let best_ao5 = session.best_ao5().unwrap();
        assert!(best_ao5.eq_aprrox(&SolveTime::new(Duration::from_millis(13_190), None), 10));
        let best_ao12 = session.best_ao12().unwrap();
        assert!(best_ao12.eq_aprrox(&SolveTime::new(Duration::from_millis(13_770), None), 10));

        session.get(6).unwrap().set_penalty(Some(Penalty::Plus2));
        session.solve_updated(6);
        session.get(8).unwrap().set_penalty(Some(Penalty::Dnf));
        session.solve_updated(8);

        let last_solve = session.last_solve().unwrap();
        let last_time = last_solve.time();
        assert!(last_time.eq_aprrox(&SolveTime::new(Duration::from_millis(15_020), None), 10));
        let last_mo3 = last_solve.mo3().unwrap();
        assert!(last_mo3.eq_aprrox(&SolveTime::new(Duration::from_millis(14_410), None), 10));
        let last_ao5 = last_solve.ao5().unwrap();
        assert!(last_ao5.eq_aprrox(&SolveTime::new(Duration::from_millis(14_410), None), 10));
        let last_ao12 = last_solve.ao12().unwrap();
        assert!(last_ao12.eq_aprrox(&SolveTime::new(Duration::from_millis(14_310), None), 10));
        let best_time = session.best_solve().unwrap().time();
        assert!(best_time.eq_aprrox(&SolveTime::new(Duration::from_millis(12_530), None), 10));
        let best_mo3 = session.best_mo3().unwrap();
        assert!(best_mo3.eq_aprrox(&SolveTime::new(Duration::from_millis(13_420), None), 10));
        let best_ao5 = session.best_ao5().unwrap();
        assert!(best_ao5.eq_aprrox(&SolveTime::new(Duration::from_millis(13_560), None), 10));
        let best_ao12 = session.best_ao12().unwrap();
        assert!(best_ao12.eq_aprrox(&SolveTime::new(Duration::from_millis(14_310), None), 10));
    }
}
