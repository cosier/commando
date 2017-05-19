// use cursive::Cursive;
// use cursive::views::{Dialog, TextView};

use db::{Database};
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
                preferences: Database::prefs(name),
                name: name,
            };

            return app;
        }
    }

    pub fn cli(&self)  {
        let processor = cli::Processor::new();
        processor.parse()
    }
}
