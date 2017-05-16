#![feature(plugin)]
#![cfg_attr(test, plugin(stainless))]

#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;

extern crate termion;

extern crate env_logger;
extern crate cursive;
extern crate jfs;
extern crate libc;
extern crate clap;

pub mod utils;
pub mod app;
pub mod project;
pub mod preferences;
pub mod cli;
pub mod db;
