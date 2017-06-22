
use jsondb::{JsonDB};

use std::path::PathBuf;
use std::collections::HashMap;
use std;

use preferences::Preferences;
use project::ProjectData;

pub struct Persistence {
    pub preferences: Preferences,
    db: JsonDB,
}

impl Persistence {

    pub fn new() ->  Persistence {
        let mut db_file = String::from(std::env::home_dir().unwrap().to_owned().to_str().unwrap());
        db_file.push_str("/.commando.json");

        let db = JsonDB::from_file(db_file);
        let key = "preferences";

        let prefs = match db.get::<Preferences>(key) {
            Ok(d) => d,
            Err(e) => {
                debug!("Creating new Preferences");

                let prefs = Preferences {
                    active_project: None,
                    projects: HashMap::new(),
                };

                db.save(prefs, key);

                prefs
            }
        };

        Persistence { db: db, preferences: prefs }
    }

    pub fn save(&self) -> bool {
        self.db.save_with_id(&self.preferences).unwrap();
        true
    }

    pub fn projects(&self) -> Vec<Box<ProjectData>> {
        let map = self.preferences.projects;
        let mut vec: Vec<Box<ProjectData>> = Vec::new();

        for (_, project) in &map {
            let bx: Box<ProjectData> = Box::new(project.clone());
            vec.push(bx);
        }

        vec
    }
}
