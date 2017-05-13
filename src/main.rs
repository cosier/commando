#![feature(custom_derive, plugin, splice)]

#[macro_use]
extern crate serde_derive;

extern crate cursive;
extern crate jfs;

#[derive(Serialize, Deserialize)]
struct Project {
    barge_root: String,
    vault_root: String,
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Preferences {
    active_project: String,
    projects: Vec<Project>
}


use jfs::Store;
use cursive::Cursive;
use cursive::views::{Dialog, TextView};

fn main() {
    println!("Commando initialising.");

    let mut cfg = jfs::Config::default();
    cfg.pretty = true;  // false is default
    cfg.single = true;
    cfg.indent = 4;

    let projects = Vec::new();
    let db = Store::new_with_cfg("commando", cfg).unwrap();
    
    let f = Preferences {
        active_project: String::from("crowdist"),
        projects: projects
    };

    let id = db.save(&f).unwrap();

    // let obj = db.get::<Preferences>(&id).unwrap();
    // db.delete(&id).unwrap();
}
