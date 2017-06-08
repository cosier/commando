pub struct Database {}

use jfs;
use jfs::Store;

use std::path::{PathBuf};
use std::collections::{HashMap};
use std;

use preferences::Preferences;
use project::ProjectData;

pub const DEFAULT_CON: &str = "commando";

impl Database {

    pub fn list_projects() -> Vec<Box<ProjectData>> {
        let map = Database::prefs().projects;
        let mut vec: Vec<Box<ProjectData>> = Vec::new();

        for (_, project) in &map {
            let bx:Box<ProjectData> = Box::new(project.copy());
            vec.push(bx);
        }

        vec
    }

    pub fn new_project(d: ProjectData) -> bool {
        true
    }

    pub fn prefs() -> Preferences {
        Database::load(&mut Database::conn(DEFAULT_CON))
    }

    pub fn conn(name: &str) -> Store {
        let mut cfg = jfs::Config::default();
        cfg.pretty = true;
        cfg.single = true;
        cfg.indent = 4;

        let store = Store::new_with_cfg(name, cfg).unwrap();
        store
    }

    pub fn save(prefs: Preferences) -> bool {
        Database::db().save_with_id(&prefs, DEFAULT_CON).unwrap();
        true
    }

    fn load(db: &mut Store) -> Preferences {
        match db.get::<Preferences>(DEFAULT_CON) {
            Ok(d) => d,
            Err(e) => {
                debug!("Not loaded from disk:\n New particle created \n{}\n", e);

                let prefs = Preferences {
                    active_project: None,
                    projects: HashMap::new()
                };

                db.save_with_id(&prefs, DEFAULT_CON).unwrap();
                prefs
            }
        }
    }

    fn db() -> Store {
        let mut db = Database::conn(DEFAULT_CON);
        let mut db_file = String::from(
            std::env::home_dir().unwrap().to_owned().to_str().unwrap());

        db_file.push_str(&format!("/.{}.json", DEFAULT_CON)[..]);
        db.set_path(PathBuf::from(&db_file[..]));
        db
    }
}
