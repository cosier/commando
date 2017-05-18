use db::Database as DB;

use std;
use std::fmt;
use std::env::current_dir;
use std::path::PathBuf;

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

pub fn create_project(name: &str, path: PathBuf) {
    debug!("creating project: {}", name);
    debug!("destination: {}", path.to_str().unwrap());
}

pub fn promote_project(name: &str) {
}

pub fn purge_project(name: &str) {
}

pub fn info_project(name: &str) {
}

pub fn setup_project(name: &str) {
}

pub fn project_service_enable(project_name: &str, service_name: &str) {
}

pub fn project_service_disable(project_name: &str, service_name: &str) {
}

pub fn project_service_start(project_name: &str, service_name: &str) {
}

pub fn project_service_stop(project_name: &str, service_name: &str) {
}

pub fn project_service_restart(project_name: &str, service_name: &str) {
}

pub fn project_service_logs(project_name: &str, service_name: &str) {
}

pub fn project_service_env(project_name: &str, service_name: &str) {
}
