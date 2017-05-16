#![feature(plugin)]
#![cfg_attr(test, plugin(stainless))]

#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;

extern crate env_logger;
extern crate cursive;
extern crate jfs;
extern crate libc;
extern crate clap;

pub mod app;
pub mod project;
pub mod preferences;
