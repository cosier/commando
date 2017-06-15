// use cursive::Cursive;
// use cursive::views::{Dialog, TextView};

use db::{Database};
use preferences::{Preferences};
use cli;

pub struct App {
    preferences: Preferences,
}

impl App {
    pub fn new() -> App {
        {
            let app = App {
                preferences: Database::prefs(),
            };

            return app;
        }
    }

    pub fn startup(&self)  {
        debug!("startup {:?}", self.preferences);
        cli::Processor::new().parse();
    }
}
