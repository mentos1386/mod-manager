/* application.rs
 *
 * Copyright 2023 Tine
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
use gtk::{gdk, prelude::*};
use gtk::{gio, glib};
use tracing::{debug, info};

use crate::config::VERSION;
use crate::config::{APP_ID, PKGDATADIR, PROFILE};
use crate::windows::main::ModManagerWindowMain;
use crate::windows::main::Welcome;

mod imp {

    use super::*;

    #[derive(Debug, Default)]
    pub struct ModManagerApplication {}

    #[glib::object_subclass]
    impl ObjectSubclass for ModManagerApplication {
        const NAME: &'static str = "ModManagerApplication";
        type Type = super::ModManagerApplication;
        type ParentType = adw::Application;
    }

    impl ObjectImpl for ModManagerApplication {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.set_accels_for_action("app.quit", &["<primary>q"]);

            //let settings = ModManagerSettings::default();
            // let games = settings.games();

            //let settings = gio::Settings::new(APP_ID);
            //let games = settings.string("games");

            //println!("Games: {:?}", games);
        }
    }

    impl ApplicationImpl for ModManagerApplication {
        // We connect to the activate callback to create a window when the application
        // has been launched. Additionally, this callback notifies us when the user
        // tries to launch a "second instance" of the application. When they try
        // to do that, we'll just present any existing window.
        fn activate(&self) {
            debug!("GtkApplication<ModManager>::activate");
            let application = self.obj();
            // Get the current window or create one if necessary
            let window = if let Some(window) = application.active_window() {
                window
            } else {
                // TODO: Figure our if show welcome or games_and_mods.
                let welcome = Welcome::new();
                let window = ModManagerWindowMain::new(&*application, &welcome.upcast());
                window.upcast()
            };

            // Ask the window manager/compositor to present the window
            window.present();
        }

        fn startup(&self) {
            debug!("GtkApplication<ModManager>::startup");
            self.parent_startup();
            let app = self.obj();

            // Set icons for shell
            gtk::Window::set_default_icon_name(APP_ID);

            app.setup_css();
            app.setup_gactions();
        }
    }

    impl GtkApplicationImpl for ModManagerApplication {}
    impl AdwApplicationImpl for ModManagerApplication {}
}

glib::wrapper! {
    pub struct ModManagerApplication(ObjectSubclass<imp::ModManagerApplication>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl ModManagerApplication {
    pub fn new(flags: &gio::ApplicationFlags) -> Self {
        glib::Object::builder()
            .property("application-id", APP_ID)
            .property("flags", flags)
            .build()
    }

    pub fn run(&self) -> glib::ExitCode {
        info!("Mod Manager ({})", APP_ID);
        info!("Version: {} ({})", VERSION, PROFILE);
        info!("Datadir: {}", PKGDATADIR);

        ApplicationExtManual::run(self)
    }

    fn setup_gactions(&self) {
        let quit_action = gio::ActionEntry::builder("quit")
            .activate(move |app: &Self, _, _| app.quit())
            .build();
        let about_action = gio::ActionEntry::builder("about")
            .activate(move |app: &Self, _, _| app.show_about())
            .build();
        self.add_action_entries([quit_action, about_action]);
    }

    fn setup_css(&self) {
        let provider = gtk::CssProvider::new();
        provider.load_from_resource("/dev/mnts/ModManager/style.css");
        if let Some(display) = gdk::Display::default() {
            gtk::style_context_add_provider_for_display(
                &display,
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
    }

    fn show_about(&self) {
        let window = self.active_window().unwrap();
        let about = adw::AboutWindow::builder()
            .transient_for(&window)
            .application_name("mod-manager")
            .application_icon(APP_ID)
            .developer_name("Tine Jozelj")
            .version(VERSION)
            .developers(vec!["Tine Jozelj <tine@tjo.space>"])
            .copyright("© 2023 Tine Jozelj and other developers")
            .build();

        about.present();
    }
}
