use crate::data::{SessionItem, SolveData, SolveStatistic, SolveTime};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

#[allow(clippy::enum_variant_names)]
#[doc(hidden)]
mod imp {
    use std::{cell::RefCell, marker::PhantomData};

    use gtk::glib::subclass::{Signal, SignalType};
    use once_cell::sync::Lazy;

    use super::*;

    #[derive(Debug, Default, glib::Properties)]
    #[properties(wrapper_type = super::Session)]
    pub struct Session {
        pub(super) solve_list: RefCell<Vec<SessionItem>>,
        pub(super) handler_list: RefCell<Vec<glib::SignalHandlerId>>,

        #[property(get = Self::last_solve_string)]
        last_solve_string: PhantomData<String>,
        #[property(get = Self::last_mo3_string)]
        last_mo3_string: PhantomData<String>,
        #[property(get = Self::last_ao5_string)]
        last_ao5_string: PhantomData<String>,
        #[property(get = Self::last_ao12_string)]
        last_ao12_string: PhantomData<String>,
        #[property(get = Self::best_solve_string)]
        best_solve_string: PhantomData<String>,
        #[property(get = Self::best_mo3_string)]
        best_mo3_string: PhantomData<String>,
        #[property(get = Self::best_ao5_string)]
        best_ao5_string: PhantomData<String>,
        #[property(get = Self::best_ao12_string)]
        best_ao12_string: PhantomData<String>,
    }

    impl Session {
        fn last_solve_string(&self) -> String {
            self.solve_list
                .borrow()
                .last()
                .map(SessionItem::time)
                .map_or(String::default(), |t| t.to_string())
        }

        fn last_mo3_string(&self) -> String {
            self.solve_list
                .borrow()
                .last()
                .and_then(SessionItem::mo3)
                .map_or(String::default(), |t| t.to_string())
        }

        fn last_ao5_string(&self) -> String {
            self.solve_list
                .borrow()
                .last()
                .and_then(SessionItem::ao5)
                .map_or(String::default(), |t| t.to_string())
        }

        fn last_ao12_string(&self) -> String {
            self.solve_list
                .borrow()
                .last()
                .and_then(SessionItem::ao12)
                .map_or(String::default(), |t| t.to_string())
        }

        fn best_solve_string(&self) -> String {
            self.solve_list
                .borrow()
                .iter()
                .map(SessionItem::time)
                .min()
                .map_or(String::default(), |t| t.to_string())
        }

        fn best_mo3_string(&self) -> String {
            self.solve_list
                .borrow()
                .iter()
                .min_by_key(|item| item.mo3().unwrap_or(SolveTime::DNF))
                .and_then(SessionItem::mo3)
                .map_or(String::default(), |t| t.to_string())
        }

        fn best_ao5_string(&self) -> String {
            self.solve_list
                .borrow()
                .iter()
                .min_by_key(|item| item.ao5().unwrap_or(SolveTime::DNF))
                .and_then(SessionItem::ao5)
                .map_or(String::default(), |t| t.to_string())
        }

        fn best_ao12_string(&self) -> String {
            self.solve_list
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
                vec![
                    Signal::builder("solve-added")
                        .param_types(Vec::<SignalType>::new())
                        .build(),
                    Signal::builder("solve-removed")
                        .param_types(Vec::<SignalType>::new())
                        .build(),
                    Signal::builder("new-best-solve")
                        .param_types(Vec::<SignalType>::new())
                        .build(),
                    Signal::builder("new-best-mo3")
                        .param_types(Vec::<SignalType>::new())
                        .build(),
                    Signal::builder("new-best-ao5")
                        .param_types(Vec::<SignalType>::new())
                        .build(),
                    Signal::builder("new-best-ao12")
                        .param_types(Vec::<SignalType>::new())
                        .build(),
                ]
            });
            SIGNALS.as_ref()
        }
    }

    impl ListModelImpl for Session {
        fn item_type(&self) -> glib::Type {
            SessionItem::static_type()
        }

        fn n_items(&self) -> u32 {
            self.solve_list.borrow().len() as u32
        }

        fn item(&self, position: u32) -> Option<glib::Object> {
            self.solve_list
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

    /// Gets the nth solve of this session.
    pub fn get_solve(&self, index: usize) -> Option<SessionItem> {
        self.imp().solve_list.borrow().get(index).cloned()
    }

    /// Gets a slice of sequence of `n_item` solves, starting from `index`.
    pub fn get_solve_slice(&self, index: usize, n_item: usize) -> Option<Vec<SessionItem>> {
        if n_item > index + 1 {
            return None;
        }
        self.imp()
            .solve_list
            .borrow()
            .get(index + 1 - n_item..index + 1)
            .map(|s| s.to_vec())
    }

    /// Returns the index of the object in this session.
    fn get_solve_index(&self, obj: &SessionItem) -> Option<usize> {
        self.imp()
            .solve_list
            .borrow()
            .iter()
            .position(|item| item == obj)
    }

    /// Adds a solve to this session.
    pub fn add_solve(&self, solve: SolveData) -> SessionItem {
        let item = SessionItem::new(solve);
        let handler =
            item.connect_solve_time_string_notify(glib::clone!(@weak self as obj => move |solve| {
                obj.solve_updated_by_object(solve);
            }));
        self.imp().solve_list.borrow_mut().push(item.clone());
        self.imp().handler_list.borrow_mut().push(handler);

        let index = self.n_items() - 1;
        self.items_changed(index, 0, 1);
        self.solve_updated(index as usize);
        self.emit_by_name::<()>("solve-added", &[]);
        self.notify_new_best_times();
        item
    }

    /// Remove the item at this index in this session.
    pub fn remove_solve(&self, index: usize) -> Option<SessionItem> {
        let imp = self.imp();
        if index as u32 >= self.n_items() {
            return None;
        }

        let solve = imp.solve_list.borrow_mut().remove(index);
        let handler = imp.handler_list.borrow_mut().remove(index);
        solve.disconnect(handler);

        self.items_changed(index as u32, 1, 0);
        self.solve_updated(index);
        self.emit_by_name::<()>("solve-removed", &[]);
        Some(solve)
    }

    /// Remove `SessionItem` object in this session.
    pub fn remove_solve_by_object(&self, obj: &SessionItem) -> Option<SessionItem> {
        self.get_solve_index(obj).and_then(|i| self.remove_solve(i))
    }

    /// Gets the last solve of this session.
    pub fn last_solve(&self) -> Option<SessionItem> {
        self.imp().solve_list.borrow().last().cloned()
    }

    /// Gets the last solve time of this session.
    pub fn last_solve_time(&self) -> Option<SolveTime> {
        self.last_solve().map(|s| s.time())
    }

    /// Gets the last mean of 3 time of this session.
    pub fn last_mo3(&self) -> Option<SolveTime> {
        self.last_solve().and_then(|s| s.mo3())
    }

    /// Gets the last average of 5 time of this session.
    pub fn last_ao5(&self) -> Option<SolveTime> {
        self.last_solve().and_then(|s| s.ao5())
    }

    /// Gets the last average of 12 time of this session.
    pub fn last_ao12(&self) -> Option<SolveTime> {
        self.last_solve().and_then(|s| s.ao12())
    }

    /// Gets the best solve item of this session.
    pub fn best_solve(&self) -> Option<SessionItem> {
        self.imp()
            .solve_list
            .borrow()
            .iter()
            .min_by_key(|item| item.time())
            .cloned()
    }

    /// Gets the best solve time of this session.
    pub fn best_solve_time(&self) -> Option<SolveTime> {
        self.imp()
            .solve_list
            .borrow()
            .iter()
            .min_by_key(|item| item.time())
            .map(SessionItem::time)
    }

    /// Gets the best mean of 3 time of this session.
    pub fn best_mo3(&self) -> Option<SolveTime> {
        self.imp()
            .solve_list
            .borrow()
            .iter()
            .min_by_key(|item| item.mo3().unwrap_or(SolveTime::DNF))
            .and_then(SessionItem::mo3)
    }

    /// Gets the best average of 5 time of this session.
    pub fn best_ao5(&self) -> Option<SolveTime> {
        self.imp()
            .solve_list
            .borrow()
            .iter()
            .min_by_key(|item| item.ao5().unwrap_or(SolveTime::DNF))
            .and_then(SessionItem::ao5)
    }

    /// Gets the best average of 12 time of this session.
    pub fn best_ao12(&self) -> Option<SolveTime> {
        self.imp()
            .solve_list
            .borrow()
            .iter()
            .min_by_key(|item| item.ao12().unwrap_or(SolveTime::DNF))
            .and_then(SessionItem::ao12)
    }

    /// Gets the index of the best solve item of this session.
    pub fn best_solve_index(&self) -> Option<usize> {
        self.imp()
            .solve_list
            .borrow()
            .iter()
            .enumerate()
            .min_by_key(|&(_, item)| item.time())
            .map(|(i, _)| i)
    }

    /// Gets the index of the best mean of 3 time of this session.
    pub fn best_mo3_index(&self) -> Option<usize> {
        if self.n_items() < 3 {
            return None;
        }

        self.imp()
            .solve_list
            .borrow()
            .iter()
            .enumerate()
            .min_by_key(|&(_, item)| item.mo3().unwrap_or(SolveTime::DNF))
            .map(|(i, _)| i)
    }

    /// Gets the index of the best average of 5 time of this session.
    pub fn best_ao5_index(&self) -> Option<usize> {
        if self.n_items() < 5 {
            return None;
        }

        self.imp()
            .solve_list
            .borrow()
            .iter()
            .enumerate()
            .min_by_key(|&(_, item)| item.ao5().unwrap_or(SolveTime::DNF))
            .map(|(i, _)| i)
    }

    /// Gets the index of the best average of 12 time of this session.
    pub fn best_ao12_index(&self) -> Option<usize> {
        if self.n_items() < 12 {
            return None;
        }

        self.imp()
            .solve_list
            .borrow()
            .iter()
            .enumerate()
            .min_by_key(|&(_, item)| item.ao12().unwrap_or(SolveTime::DNF))
            .map(|(i, _)| i)
    }

    /// Computes the mean of 3 of the solve at this index.
    fn compute_mo3(&self, index: usize) -> Option<SolveTime> {
        let list = self.imp().solve_list.borrow();
        if index + 1 >= 3 {
            list.get(index - 2..index + 1)
                .and_then(|solves| solves.mean_of_n())
        } else {
            None
        }
    }

    /// Computes the average of 5 of the solve at this index.
    fn compute_ao5(&self, index: usize) -> Option<SolveTime> {
        let list = self.imp().solve_list.borrow();
        if index + 1 >= 5 {
            list.get(index - 4..index + 1)
                .and_then(|solves| solves.average_of_n())
        } else {
            None
        }
    }

    /// Computes the average of 12 the solve at this index.
    fn compute_ao12(&self, index: usize) -> Option<SolveTime> {
        let list = self.imp().solve_list.borrow();
        if index + 1 >= 12 {
            list.get(index - 11..index + 1)
                .and_then(|solves| solves.average_of_n())
        } else {
            None
        }
    }

    /// Updates the mean of 3 of the solve at this index.
    fn update_mo3(&self, index: usize) {
        let mo3 = self.compute_mo3(index);
        if let Some(item) = self.imp().solve_list.borrow().get(index) {
            item.set_mo3(mo3);
        }
    }

    /// Updates the average of 5 of the solve at this index.
    fn update_ao5(&self, index: usize) {
        let ao5 = self.compute_ao5(index);
        if let Some(item) = self.imp().solve_list.borrow().get(index) {
            item.set_ao5(ao5);
        }
    }

    /// Updates the average of 12 the solve at this index.
    fn update_ao12(&self, index: usize) {
        let ao12 = self.compute_ao12(index);
        if let Some(item) = self.imp().solve_list.borrow().get(index) {
            item.set_ao12(ao12);
        }
    }

    /// Notifies update on an item in this index.
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
        self.notify_statistics_changed();
    }

    /// Notifies update on a `SessionItem` object in this session.
    pub fn solve_updated_by_object(&self, obj: &SessionItem) {
        if let Some(index) = self.get_solve_index(obj) {
            self.solve_updated(index);
        } else {
            log::warn!("PtSessionItem object is not in Session");
        }
    }

    /// Notify changes in statistics of the session.
    pub fn notify_statistics_changed(&self) {
        self.notify_last_solve_string();
        self.notify_last_mo3_string();
        self.notify_last_ao5_string();
        self.notify_last_ao12_string();
        self.notify_best_solve_string();
        self.notify_best_mo3_string();
        self.notify_best_ao5_string();
        self.notify_last_ao12_string();
    }

    /// Notifies a new best time.
    fn notify_new_best_times(&self) {
        let last_i = self.n_items() as usize - 1;

        if self.best_solve_index().is_some_and(|i| i == last_i) {
            self.emit_by_name::<()>("new-best-solve", &[]);
        }

        if self.best_mo3_index().is_some_and(|i| i == last_i) {
            self.emit_by_name::<()>("new-best-mo3", &[]);
        }

        if self.best_ao5_index().is_some_and(|i| i == last_i) {
            self.emit_by_name::<()>("new-best-ao5", &[]);
        }

        if self.best_ao12_index().is_some_and(|i| i == last_i) {
            self.emit_by_name::<()>("new-best-ao12", &[]);
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
        });
    }

    fn build_test_session() -> Session {
        let session = Session::new();
        add_dummy_solve(
            &session,
            SolveTime::new(Duration::from_millis(13_440), Penalty::Ok),
        );
        add_dummy_solve(
            &session,
            SolveTime::new(Duration::from_millis(14_320), Penalty::Ok),
        );
        add_dummy_solve(
            &session,
            SolveTime::new(Duration::from_millis(15_900), Penalty::Ok),
        );
        add_dummy_solve(
            &session,
            SolveTime::new(Duration::from_millis(12_530), Penalty::Ok),
        );
        add_dummy_solve(
            &session,
            SolveTime::new(Duration::from_millis(13_080), Penalty::Ok),
        );
        add_dummy_solve(
            &session,
            SolveTime::new(Duration::from_millis(14_650), Penalty::Ok),
        );
        add_dummy_solve(
            &session,
            SolveTime::new(Duration::from_millis(13_540), Penalty::Ok),
        );
        add_dummy_solve(
            &session,
            SolveTime::new(Duration::from_millis(12_940), Penalty::Ok),
        );
        add_dummy_solve(
            &session,
            SolveTime::new(Duration::from_millis(12_110), Penalty::Ok),
        );
        add_dummy_solve(
            &session,
            SolveTime::new(Duration::from_millis(13_890), Penalty::Ok),
        );
        add_dummy_solve(
            &session,
            SolveTime::new(Duration::from_millis(14_330), Penalty::Ok),
        );
        add_dummy_solve(
            &session,
            SolveTime::new(Duration::from_millis(15_020), Penalty::Ok),
        );
        session
    }

    #[test]
    fn verify_last_solve() {
        let session = build_test_session();
        let last_solve = session.last_solve().unwrap();

        let last_time = last_solve.time();

        assert!(last_time.eq_aprrox(
            &SolveTime::new(Duration::from_millis(15_020), Penalty::Ok),
            10
        ));
    }

    #[test]
    fn verify_last_mo3() {
        let session = build_test_session();
        let last_solve = session.last_solve().unwrap();

        let last_mo3 = last_solve.mo3().unwrap();

        assert!(last_mo3.eq_aprrox(
            &SolveTime::new(Duration::from_millis(14_410), Penalty::Ok),
            10
        ));
    }

    #[test]
    fn verify_last_ao5() {
        let session = build_test_session();
        let last_solve = session.last_solve().unwrap();

        let last_ao5 = last_solve.ao5().unwrap();

        assert!(last_ao5.eq_aprrox(
            &SolveTime::new(Duration::from_millis(13_720), Penalty::Ok),
            10
        ));
    }

    #[test]
    fn verify_last_ao12() {
        let session = build_test_session();
        let last_solve = session.last_solve().unwrap();

        let last_ao12 = last_solve.ao12().unwrap();

        assert!(last_ao12.eq_aprrox(
            &SolveTime::new(Duration::from_millis(13_770), Penalty::Ok),
            10
        ));
    }

    #[test]
    fn verify_best_solve() {
        let session = build_test_session();

        let best_time = session.best_solve().unwrap().time();

        assert!(best_time.eq_aprrox(
            &SolveTime::new(Duration::from_millis(12_110), Penalty::Ok),
            10
        ));
    }

    #[test]
    fn verify_best_mo3() {
        let session = build_test_session();

        let best_mo3 = session.best_mo3().unwrap();

        assert!(best_mo3.eq_aprrox(
            &SolveTime::new(Duration::from_millis(12_860), Penalty::Ok),
            10
        ));
    }

    #[test]
    fn verify_best_ao5() {
        let session = build_test_session();

        let best_ao5 = session.best_ao5().unwrap();

        assert!(best_ao5.eq_aprrox(
            &SolveTime::new(Duration::from_millis(13_190), Penalty::Ok),
            10
        ));
    }

    #[test]
    fn verify_best_ao12() {
        let session = build_test_session();

        let best_ao12 = session.best_ao12().unwrap();

        assert!(best_ao12.eq_aprrox(
            &SolveTime::new(Duration::from_millis(13_770), Penalty::Ok),
            10
        ));
    }

    fn build_and_modify_test_session() -> Session {
        let session = build_test_session();
        session.get_solve(6).unwrap().set_penalty(Penalty::Plus2);
        session.get_solve(8).unwrap().set_penalty(Penalty::Dnf);
        session
    }

    #[test]
    fn verify_last_solve_after_modification() {
        let session = build_and_modify_test_session();
        let last_solve = session.last_solve().unwrap();

        let last_time = last_solve.time();

        assert!(last_time.eq_aprrox(
            &SolveTime::new(Duration::from_millis(15_020), Penalty::Ok),
            10
        ));
    }

    #[test]
    fn verify_last_mo3_after_modification() {
        let session = build_and_modify_test_session();
        let last_solve = session.last_solve().unwrap();

        let last_mo3 = last_solve.mo3().unwrap();

        assert!(last_mo3.eq_aprrox(
            &SolveTime::new(Duration::from_millis(14_410), Penalty::Ok),
            10
        ));
    }

    #[test]
    fn verify_last_ao5_after_modification() {
        let session = build_and_modify_test_session();
        let last_solve = session.last_solve().unwrap();

        let last_ao5 = last_solve.ao5().unwrap();

        assert!(last_ao5.eq_aprrox(
            &SolveTime::new(Duration::from_millis(14_410), Penalty::Ok),
            10
        ));
    }

    #[test]
    fn verify_last_ao12_after_modification() {
        let session = build_and_modify_test_session();
        let last_solve = session.last_solve().unwrap();

        let last_ao12 = last_solve.ao12().unwrap();

        assert!(last_ao12.eq_aprrox(
            &SolveTime::new(Duration::from_millis(14_310), Penalty::Ok),
            10
        ));
    }

    #[test]
    fn verify_best_solve_after_modification() {
        let session = build_and_modify_test_session();

        let best_time = session.best_solve().unwrap().time();

        assert!(best_time.eq_aprrox(
            &SolveTime::new(Duration::from_millis(12_530), Penalty::Ok),
            10
        ));
    }

    #[test]
    fn verify_best_mo3_after_modification() {
        let session = build_and_modify_test_session();

        let best_mo3 = session.best_mo3().unwrap();

        assert!(best_mo3.eq_aprrox(
            &SolveTime::new(Duration::from_millis(13_420), Penalty::Ok),
            10
        ));
    }

    #[test]
    fn verify_best_ao5_after_modification() {
        let session = build_and_modify_test_session();

        let best_ao5 = session.best_ao5().unwrap();

        assert!(best_ao5.eq_aprrox(
            &SolveTime::new(Duration::from_millis(13_560), Penalty::Ok),
            10
        ));
    }

    #[test]
    fn verify_best_ao12_after_modification() {
        let session = build_and_modify_test_session();

        let best_ao12 = session.best_ao12().unwrap();

        assert!(best_ao12.eq_aprrox(
            &SolveTime::new(Duration::from_millis(14_310), Penalty::Ok),
            10
        ));
    }

    fn build_and_remove_from_test_session() -> Session {
        let session = build_test_session();
        session.remove_solve(8).unwrap();
        session
    }

    #[test]
    fn verify_last_solve_after_remove_from() {
        let session = build_and_remove_from_test_session();
        let last_solve = session.last_solve().unwrap();

        let last_time = last_solve.time();

        assert!(last_time.eq_aprrox(
            &SolveTime::new(Duration::from_millis(15_020), Penalty::Ok),
            10
        ));
    }

    #[test]
    fn verify_last_mo3_after_remove_from() {
        let session = build_and_remove_from_test_session();
        let last_solve = session.last_solve().unwrap();

        let last_mo3 = last_solve.mo3().unwrap();

        assert!(last_mo3.eq_aprrox(
            &SolveTime::new(Duration::from_millis(14_410), Penalty::Ok),
            10
        ));
    }

    #[test]
    fn verify_last_ao5_after_remove_from() {
        let session = build_and_remove_from_test_session();
        let last_solve = session.last_solve().unwrap();

        let last_ao5 = last_solve.ao5().unwrap();

        assert!(last_ao5.eq_aprrox(
            &SolveTime::new(Duration::from_millis(13_920), Penalty::Ok),
            10
        ));
    }

    #[test]
    fn verify_last_ao12_after_remove_from() {
        let session = build_and_remove_from_test_session();
        let last_solve = session.last_solve().unwrap();

        let last_ao12 = last_solve.ao12();

        assert!(last_ao12.is_none());
    }

    #[test]
    fn verify_best_solve_after_remove_from() {
        let session = build_and_remove_from_test_session();

        let best_time = session.best_solve().unwrap().time();

        assert!(best_time.eq_aprrox(
            &SolveTime::new(Duration::from_millis(12_530), Penalty::Ok),
            10
        ));
    }

    #[test]
    fn verify_best_mo3_after_remove_from() {
        let session = build_and_remove_from_test_session();

        let best_mo3 = session.best_mo3().unwrap();

        assert!(best_mo3.eq_aprrox(
            &SolveTime::new(Duration::from_millis(13_420), Penalty::Ok),
            10
        ));
    }

    #[test]
    fn verify_best_ao5_after_remove_from() {
        let session = build_and_remove_from_test_session();

        let best_ao5 = session.best_ao5().unwrap();

        assert!(best_ao5.eq_aprrox(
            &SolveTime::new(Duration::from_millis(13_190), Penalty::Ok),
            10
        ));
    }

    #[test]
    fn verify_best_ao12_after_remove_from() {
        let session = build_and_remove_from_test_session();

        let best_ao12 = session.best_ao12();

        assert!(best_ao12.is_none())
    }
}
