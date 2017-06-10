#![feature(plugin)]
#![cfg_attr(test, plugin(stainless))]
#![recursion_limit = "16384"]

#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate error_chain;
// #[macro_use] extern crate lazy_static;

extern crate serde_json;
extern crate serde_yaml;

extern crate termion;
extern crate slug;

extern crate env_logger;
extern crate cursive;
extern crate jfs;
extern crate libc;
extern crate clap;
extern crate git2;
extern crate curl;
extern crate term;
extern crate toml;
extern crate semver;
extern crate url;

extern crate rustc_serialize;

pub mod views;
pub mod utils;
pub mod app;
pub mod project;
pub mod service;
pub mod repository;
pub mod environment;
pub mod preferences;
pub mod cli;
pub mod db;
pub mod git;
