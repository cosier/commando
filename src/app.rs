// use cursive::Cursive;
// use cursive::views::{Dialog, TextView};

use log::LogLevel;
use project::ProjectData;
use preferences::Preferences;
use db::Database;
use cli;

pub struct App<'a> {
    name: &'a str,
    preferences: Preferences,
}

impl<'a> App<'a> {
    pub fn new(name: &str) -> App {
        {
            let app = App {
                preferences: Database::prefs(name),
                name: name,
            };

            return app;
        }
    }

    pub fn cli(&self)  {
        cli::parse();
    }
}
