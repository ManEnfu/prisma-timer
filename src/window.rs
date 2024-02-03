use crate::data::TimerState;
use crate::{data, ui};
use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gdk, gio, glib};

#[doc(hidden)]
mod imp {
    use std::cell::{Cell, RefCell};

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
        pub timer_state_machine: RefCell<Option<data::SimpleTimerStateMachine>>,
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
    }

    impl PrismaTimerWindow {
        fn set_timer_state_machine(&self, v: Option<data::SimpleTimerStateMachine>) {
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
                    glib::closure_local!(@strong obj => move |sm: &data::SimpleTimerStateMachine| {
                        obj.timer_state_changed_cb(sm);
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

            obj.set_timer_state_machine(data::SimpleTimerStateMachine::new());

            obj.setup_gactions();
            obj.setup_event_controllers();
            obj.setup_list();
        }
    }

    impl WidgetImpl for PrismaTimerWindow {}
    impl WindowImpl for PrismaTimerWindow {}
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

    fn timer_state_changed_cb(&self, sm: &data::SimpleTimerStateMachine) {
        let imp = self.imp();

        if sm.running() {
            if imp.split_view.is_collapsed() {
                imp.split_view.set_show_sidebar(false);
            }
            self.set_focus_mode(true);
        } else {
            self.set_focus_mode(false);
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

    #[template_callback]
    fn list_view_activated_cb(&self, position: u32, _list_view: &gtk::ListView) {
        let session = self.session().unwrap();
        let index = session.n_items() - position - 1;

        let dialog = ui::SolveDialog::new(session, index);
        dialog.set_transient_for(Some(self));
        dialog.present();
    }
}
