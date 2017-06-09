// use cursive::Cursive;
// use cursive::views::{Dialog, TextView};

use db::{Database};
use preferences::{Preferences};
use cli;

pub struct App<'a> {
    name: &'a str,
    preferences: Preferences,
}

impl<'a> App<'a> {
    pub fn new(name: &str) -> App {
        {
            let app = App {
                preferences: Database::prefs(),
                name: name,
            };

            return app;
        }
    }

    pub fn startup(&self)  {
        debug!("{} startup {:?}", self.name, self.preferences);
        cli::Processor::new().parse();
    }
}
