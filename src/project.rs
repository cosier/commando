use db::Database as DB;

use std;
use std::fmt;
use std::env::current_dir;
use std::path::PathBuf;

use utils::{exit, check_path_exists};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectData {
    pub barge_root: String,
    pub vault_root: String,
    pub name: String,
}

impl ProjectData {
    pub fn copy (&self) -> ProjectData {
        return ProjectData {
            barge_root: self.barge_root.clone(),
            vault_root: self.vault_root.clone(),
            name: self.name.clone(),
        }
    }
}

impl fmt::Display for ProjectData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[project:{}:{}]", self.name, self.barge_root)
    }
}

fn assert_project_exists(name: &str) {
}

pub fn list() -> Vec<Box<ProjectData>> {
    let projects = DB::list_projects();
    projects
}

pub fn active_project() -> Option<String> {
    DB::prefs("commando").active_project
}

/// Creates a Project model definition and directory structure
pub fn create_project(name: &str, path: PathBuf) -> bool {

    if check_path_exists(&path) {
        println!("Path Error: Found existing directory:\n{}",
                 path.to_str().unwrap());
        exit();
    }

    if check_project_exists(name) {
        println!("Existing Project Error: Cannot create project with name:\n{}",
                 path.to_str().unwrap());
        exit();
    }

    true
}

pub fn promote_project(name: &str) -> bool {
    true
}

pub fn purge_project(name: &str) -> bool {
    true
}

pub fn info_project(name: &str) -> bool{
    true
}

pub fn setup_project(name: &str) -> bool{
    true
}

/////////////////////////////////////////////////
// Private

fn check_project_exists(name: &str) -> bool {
   false
}
