use gio::glib;
use gsettings_macro::gen_settings;
use gtk::gio;
use std::collections::HashMap;

use crate::config::APP_ID;

#[gen_settings(file = "./data/dev.mnts.ModManager.gschema.xml.in")]
pub struct ModManagerSettings;

impl Default for ModManagerSettings {
    fn default() -> Self {
        Self::new(APP_ID)
    }
}
