use gio::glib;
use gsettings_macro::gen_settings;
use gtk::gio;
use std::collections::HashMap;

use crate::config::APP_ID;

#[gen_settings(file = "./data/dev.mnts.ModManager.gschema.xml.in")]
#[gen_settings_define(
    key_name = "games",
    arg_type = "HashMap<String, String>",
    ret_type = "HashMap<String, String>"
)]
pub struct ModManagerSettings;

impl Default for ModManagerSettings {
    fn default() -> Self {
        Self::new(APP_ID)
    }
}
