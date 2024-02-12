use crate::data::TimerState;
use crate::{config, data, ui};
use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gdk, gio, glib};

#[doc(hidden)]
mod imp {
    use std::cell::{Cell, RefCell};

    use once_cell::sync::OnceCell;

    use crate::util::TemplateCallbacks;

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate, glib::Properties)]
    #[template(resource = "/io/github/manenfu/PrismaTimer/ui/window.ui")]
    #[properties(wrapper_type = super::PrismaTimerWindow)]
    pub struct PrismaTimerWindow {
        #[template_child]
        pub toast_overlay: TemplateChild<adw::ToastOverlay>,
        #[template_child]
        pub sidebar_header_bar: TemplateChild<adw::HeaderBar>,
        #[template_child]
        pub content_header_bar: TemplateChild<adw::HeaderBar>,
        #[template_child]
        pub timer_face: TemplateChild<ui::TimerFace>,

        #[template_child]
        pub split_view: TemplateChild<adw::OverlaySplitView>,

        #[template_child]
        pub sidebar_stack: TemplateChild<gtk::Stack>,
        #[template_child]
        pub list_view: TemplateChild<gtk::ListView>,

        /// The state machine is shared between widgets within this window.
        #[property(get, set = Self::set_timer_state_machine)]
        pub timer_state_machine: RefCell<Option<data::TimerStateMachine>>,
        timer_state_machine_handlers: RefCell<Vec<glib::SignalHandlerId>>,

        #[property(get, set)]
        pub session: RefCell<Option<data::Session>>,
        #[property(get, set)]
        pub session_sort_model: RefCell<Option<gtk::SortListModel>>,
        #[property(get, set)]
        pub session_selection_model: RefCell<Option<gtk::NoSelection>>,

        /// If set to true, this would hide most widgets aside from ones
        /// related to timing (i.e. sidebar).
        #[property(get, set)]
        pub focus_mode: Cell<bool>,
        #[property(get, set)]
        pub should_collapse: Cell<bool>,

        pub(super) settings: OnceCell<gio::Settings>,
    }

    impl PrismaTimerWindow {
        fn set_timer_state_machine(&self, v: Option<data::TimerStateMachine>) {
            let obj = self.obj();
            let mut handlers = self.timer_state_machine_handlers.borrow_mut();

            if let Some(osm) = self.timer_state_machine.take() {
                for id in handlers.drain(..) {
                    osm.disconnect(id);
                }
            }

            if let Some(sm) = &v {
                handlers.push(sm.connect_closure(
                    "state-changed",
                    false,
                    glib::closure_local!(@strong obj => move |sm: &data::TimerStateMachine| {
                        obj.timer_state_changed_cb(sm.state());
                    }),
                ));
            }
            self.timer_state_machine.replace(v);
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PrismaTimerWindow {
        const NAME: &'static str = "PrismaTimerWindow";
        type Type = super::PrismaTimerWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_instance_callbacks();
            TemplateCallbacks::bind_template_callbacks(klass);

            klass.install_action("sidebar.hide", None, move |obj, _, _| {
                let imp = obj.imp();
                if imp.split_view.is_collapsed() {
                    imp.split_view.set_show_sidebar(false);
                }
            })
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[glib::derived_properties]
    impl ObjectImpl for PrismaTimerWindow {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            obj.set_timer_state_machine(data::TimerStateMachine::new());

            obj.setup_settings();
            obj.setup_gactions();
            obj.setup_event_controllers();
            obj.setup_list();

            obj.load_window_size();
        }
    }

    impl WidgetImpl for PrismaTimerWindow {}

    impl WindowImpl for PrismaTimerWindow {
        fn close_request(&self) -> glib::Propagation {
            let obj = self.obj();

            let result = obj.save_window_size();
            if let Err(e) = result {
                log::error!("Failed to save window state. cause: {}", e);
            }

            glib::Propagation::Proceed
        }
    }

    impl ApplicationWindowImpl for PrismaTimerWindow {}
    impl AdwApplicationWindowImpl for PrismaTimerWindow {}
}

glib::wrapper! {
    pub struct PrismaTimerWindow(ObjectSubclass<imp::PrismaTimerWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

#[gtk::template_callbacks]
impl PrismaTimerWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }

    fn setup_settings(&self) {
        let imp = self.imp();

        let settings = gio::Settings::new(config::APP_ID);
        imp.settings
            .set(settings)
            .expect("`settings` should not be set before `setup_settings` is called");
    }

    fn setup_gactions(&self) {
        let shortcuts_window =
            gtk::Builder::from_resource("/io/github/manenfu/PrismaTimer/ui/shortcuts_window.ui")
                .object::<gtk::ShortcutsWindow>("shortcuts_window")
                .expect("Error building shortcuts window.");
        self.set_help_overlay(Some(&shortcuts_window));
    }

    fn setup_event_controllers(&self) {
        // Focus on timer face if user clicks on anywhere on window that has no
        // interactable widgets.
        let gestures = gtk::GestureClick::new();
        gestures.set_touch_only(false);
        gestures.set_button(gdk::BUTTON_PRIMARY);
        gestures.set_propagation_phase(gtk::PropagationPhase::Capture);
        gestures.connect_pressed(glib::clone!(@weak self as obj => move |_, _, _, _| {
            obj.imp().timer_face.grab_focus();
        }));
        self.add_controller(gestures);
    }

    fn setup_list(&self) {
        let imp = self.imp();

        let session = data::Session::new();
        self.set_session(session.clone());

        session.connect_items_changed(
            glib::clone!(@weak self as obj => move |_, position, removed, added| {
                obj.session_items_changed_cb(position, removed, added);
            }),
        );

        session.connect_closure(
            "solve-added",
            false,
            glib::closure_local!(@strong self as obj => move |_: &data::Session| {
                obj.session_solve_added_cb();
            }),
        );

        session.connect_closure(
            "solve-removed",
            false,
            glib::closure_local!(@strong self as obj => move |_: &data::Session| {
                obj.session_solve_removed_cb();
            }),
        );

        session.connect_closure(
            "new-best-solve",
            false,
            glib::closure_local!(@strong self as obj => move |_: &data::Session| {
                obj.session_new_best_solve_cb();
            }),
        );

        session.connect_closure(
            "new-best-ao5",
            false,
            glib::closure_local!(@strong self as obj => move |_: &data::Session| {
                obj.session_new_best_ao5_cb();
            }),
        );

        session.connect_closure(
            "new-best-ao12",
            false,
            glib::closure_local!(@strong self as obj => move |_: &data::Session| {
                obj.session_new_best_ao12_cb();
            }),
        );

        let sort_model = gtk::SortListModel::new(
            Some(session),
            Some(gtk::CustomSorter::new(|a, b| {
                let a = a.downcast_ref::<data::SessionItem>().unwrap();
                let b = b.downcast_ref::<data::SessionItem>().unwrap();
                b.timestamp().cmp(&a.timestamp()).into()
            })),
        );
        self.set_session_sort_model(sort_model.clone());

        let selection_model = gtk::NoSelection::new(Some(sort_model));
        self.set_session_selection_model(selection_model.clone());

        let factory = gtk::SignalListItemFactory::new();
        factory.connect_setup(glib::clone!(@weak self as view => move |_, item| {
            let row = ui::SessionItemRow::new();
            let item = item.downcast_ref::<gtk::ListItem>().unwrap();
            item.set_child(Some(&row));
            item.bind_property("item", &row, "item").sync_create().build();
        }));

        imp.list_view.set_model(Some(&selection_model));
        imp.list_view.set_factory(Some(&factory));
    }

    fn settings(&self) -> &gio::Settings {
        self.imp()
            .settings
            .get()
            .expect("`settings` should be set by `setup_settings` first")
    }

    fn save_window_size(&self) -> Result<(), glib::BoolError> {
        let settings = self.settings();

        let height = self.height();
        let width = self.width();
        let is_maximized = self.is_maximized();

        settings.set_int("window-height", height)?;
        settings.set_int("window-width", width)?;
        settings.set_boolean("window-is-maximized", is_maximized)?;

        Ok(())
    }

    fn load_window_size(&self) {
        let settings = self.settings();

        let height = settings.int("window-height");
        let width = settings.int("window-width");
        let is_maximized = settings.boolean("window-is-maximized");

        self.set_default_size(width, height);
        self.set_maximized(is_maximized);
    }

    fn timer_state_changed_cb(&self, state: TimerState) {
        let imp = self.imp();

        match state {
            TimerState::Ready | TimerState::Timing { .. } => {
                if imp.split_view.is_collapsed() {
                    imp.split_view.set_show_sidebar(false);
                }
                self.set_focus_mode(true);
            }
            _ => {
                self.set_focus_mode(false);
            }
        }
    }

    fn session_items_changed_cb(&self, _position: u32, _removed: u32, _added: u32) {
        let imp = self.imp();
        if self.session().map_or(0, |s| s.n_items()) > 0 {
            imp.sidebar_stack.set_visible_child_name("list");
        } else {
            imp.sidebar_stack.set_visible_child_name("empty");
        }
    }

    fn session_solve_added_cb(&self) {
        let imp = self.imp();
        imp.list_view.scroll_to(0, gtk::ListScrollFlags::NONE, None);
    }

    fn session_solve_removed_cb(&self) {
        let imp = self.imp();
        imp.toast_overlay
            .add_toast(adw::Toast::new("Solve Removed"));
    }

    fn session_new_best_solve_cb(&self) {
        let imp = self.imp();
        imp.toast_overlay
            .add_toast(adw::Toast::new("New Best Solve"));
    }

    fn session_new_best_ao5_cb(&self) {
        let imp = self.imp();
        imp.toast_overlay.add_toast(adw::Toast::new("New Best Ao5"));
    }

    fn session_new_best_ao12_cb(&self) {
        let imp = self.imp();
        imp.toast_overlay
            .add_toast(adw::Toast::new("New Best Ao12"));
    }

    #[template_callback]
    fn list_view_activated_cb(&self, position: u32, _list_view: &gtk::ListView) {
        let session = self.session().unwrap();
        let index = session.n_items() - position - 1;

        let dialog = ui::SolveDialog::new(session, index);
        dialog.set_transient_for(Some(self));
        dialog.present();
    }
}
