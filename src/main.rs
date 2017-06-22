#![feature(plugin)]
#![cfg_attr(test, plugin(stainless))]

extern crate env_logger;
extern crate commando;

use commando::cli;

fn main() {
    env_logger::init().unwrap();
    cli::Processor::new().parse();
}
