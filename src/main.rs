#![feature(custom_derive, plugin, splice)]

#[macro_use]
extern crate serde_derive;

extern crate cursive;
extern crate jfs;
extern crate libc;

use libc::getcwd;
use std::path::{Path,PathBuf};
use std::collections::{HashMap};
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::os::unix;
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug)]
struct Project {
    barge_root: String,
    vault_root: String,
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Preferences {
    active_project: String,
    projects: HashMap<String,Project>
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

    let projects = HashMap::new();
    let app_name = "commando";

    let mut db = Store::new_with_cfg(app_name, cfg).unwrap();

    let mut current = String::from(
        std::env::home_dir().unwrap().to_owned().to_str().unwrap());
    current.push_str("/.commando.json");

    let pbuf: PathBuf = PathBuf::from(&current[..]);

    db.set_path(pbuf.clone());

    let mut data: Preferences = match db.get::<Preferences>("commando") {
        Ok(d) => d,
        Err(e) => {
            println!("Preferences: Not loaded from disk - {}", e);
            let prefs = Preferences {
                active_project: String::from("crowdist"),
                projects: projects
            };

            db.save_with_id(&prefs, app_name).unwrap();
            prefs
        }
    };

    println!("data: {}", data.active_project);

    match cat(pbuf) {
        Ok(s) => println!("\n{}\n",s),
        Err(why) => println!("Could not read commando db: {} \npath: {}", why, current)
    };
}

fn cat(path: PathBuf) -> io::Result<String> {
    println!("\n-----\ncats: {}", path.to_str().unwrap());
    let mut f = try!(File::open(path));
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
