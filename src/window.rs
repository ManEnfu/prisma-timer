/* window.rs
 *
 * Copyright 2023 Unknown
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gio, glib};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/github/manenfu/PrismaTimer/ui/window.ui")]
    pub struct PrismaTimerWindow {
        #[template_child]
        pub sidebar_header_bar: TemplateChild<adw::HeaderBar>,
        #[template_child]
        pub content_header_bar: TemplateChild<adw::HeaderBar>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PrismaTimerWindow {
        const NAME: &'static str = "PrismaTimerWindow";
        type Type = super::PrismaTimerWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for PrismaTimerWindow {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();

            obj.setup_gactions();
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
}
