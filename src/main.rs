#![feature(plugin)]
#![cfg_attr(test, plugin(stainless))]

#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;

extern crate env_logger;
extern crate commando;

use commando::app::{App};

fn app() {
    env_logger::init().unwrap();
    App::new("commando").cli();
}

fn main() { app(); }

// #[test]
// fn test() { app(); }
