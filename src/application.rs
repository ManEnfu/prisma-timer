/* application.rs
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

use crate::config::{APP_ID, VERSION};
use crate::ui;
use crate::PrismaTimerWindow;

#[doc(hidden)]
mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct PrismaTimerApplication {}

    #[glib::object_subclass]
    impl ObjectSubclass for PrismaTimerApplication {
        const NAME: &'static str = "PrismaTimerApplication";
        type Type = super::PrismaTimerApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for PrismaTimerApplication {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_gactions();
            obj.setup_shortcuts();
        }
    }

    impl ApplicationImpl for PrismaTimerApplication {
        // We connect to the activate callback to create a window when the application
        // has been launched. Additionally, this callback notifies us when the user
        // tries to launch a "second instance" of the application. When they try
        // to do that, we'll just present any existing window.
        fn activate(&self) {
            let application = self.obj();
            // Get the current window or create one if necessary
            let window = if let Some(window) = application.active_window() {
                window
            } else {
                let window = PrismaTimerWindow::new(&*application);
                window.upcast()
            };

            // Ask the window manager/compositor to present the window
            window.present();
        }
    }

    impl GtkApplicationImpl for PrismaTimerApplication {}
    impl AdwApplicationImpl for PrismaTimerApplication {}
}

glib::wrapper! {
    pub struct PrismaTimerApplication(ObjectSubclass<imp::PrismaTimerApplication>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl PrismaTimerApplication {
    pub fn new(flags: &gio::ApplicationFlags) -> Self {
        glib::Object::builder()
            .property("application-id", APP_ID)
            .property("flags", flags)
            .build()
    }

    fn setup_gactions(&self) {
        self.add_action_entries([
            gio::ActionEntry::builder("quit")
                .activate(move |app: &Self, _, _| app.quit())
                .build(),
            gio::ActionEntry::builder("about")
                .activate(move |app: &Self, _, _| app.show_about())
                .build(),
            gio::ActionEntry::builder("preferences")
                .activate(move |app: &Self, _, _| app.show_preferences())
                .build(),
        ]);
    }

    fn setup_shortcuts(&self) {
        self.set_accels_for_action("app.quit", &["<primary>q"]);
        self.set_accels_for_action("app.preferences", &["<primary>comma"]);
        self.set_accels_for_action("win.show-help-overlay", &["<primary>question"]);
    }

    fn show_about(&self) {
        let window = self
            .active_window()
            .expect("an active window must be set for this application");

        let about = adw::AboutWindow::builder()
            .transient_for(&window)
            .application_name("Prisma Timer")
            .application_icon("io.github.manenfu.PrismaTimer")
            .developer_name("ManEnfu")
            .version(VERSION)
            .developers(vec!["ManEnfu"])
            .copyright("© 2023 ManEnfu")
            .build();

        about.present();
    }

    fn show_preferences(&self) {
        let window = self
            .active_window()
            .expect("an active window must be set for this application");

        let preferences_window = ui::PreferencesWindow::new(&window);

        preferences_window.present();
    }
}
