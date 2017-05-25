// use cursive::Cursive;
// use cursive::views::{Dialog, TextView};

use db::{Database, preferences as db_preferences};
use preferences::{Preferences};
use cli;
use clap;

pub struct App<'a> {
    name: &'a str,
    preferences: Preferences,
}

impl<'a> App<'a> {
    pub fn new(name: &str) -> App {
        {
            let app = App {
                preferences: db_preferences(),
                name: name,
            };

            return app;
        }
    }

    pub fn startup(&self)  {
        cli::Processor::new().parse();
    }
}
