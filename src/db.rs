pub struct Database {}

use jfs;
use jfs::Store;
use preferences::Preferences;
use std::path::{PathBuf};
use std::fs::{File};
use std::io;
use std::io::prelude::*;
use std::collections::{HashMap};
use std;

impl Database {

    pub fn project_by_id(id: &str) {
    }

    pub fn prefs(name: &str) -> Preferences {
        return Database::preferences(name, &mut Database::conn(name));
    }

    pub fn conn(name: &str) -> Store {
        let mut cfg = jfs::Config::default();
        cfg.pretty = true;
        cfg.single = true;
        cfg.indent = 4;

        let name = "commando";
        let store = Store::new_with_cfg(name, cfg).unwrap();
        return store;
    }

    fn preferences(name: &str, db: &mut Store) -> Preferences {
        let mut db_file = String::from(
            std::env::home_dir().unwrap().to_owned().to_str().unwrap());

        db_file.push_str(&format!("/.{}.json", name)[..]);
        db.set_path(PathBuf::from(&db_file[..]));

        return match db.get::<Preferences>(name) {
            Ok(d) => d,
            Err(e) => {
                println!("Preferences: Not loaded from disk - {}", e);
                let prefs = Preferences {
                    active_project: None,
                    projects: HashMap::new()
                };

                db.save_with_id(&prefs, name).unwrap();
                return prefs
            }
        };
    }

}