#![feature(plugin)]
#![cfg_attr(test, plugin(stainless))]

extern crate env_logger;
extern crate commando;

use commando::app::App;

fn app() {
    println!("\n");
    env_logger::init().unwrap();
    App::new().startup();
}

fn main() {
    app();
}

// #[test]
// fn test() { app(); }
