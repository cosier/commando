use db::Database as DB;
use std::fmt;

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
        write!(f, "Project: {} @ {}", self.name, self.barge_root)
    }
}

pub fn list() -> Vec<Box<ProjectData>> {
    let projects = DB::list_projects();
    projects
}

pub fn active_id() -> &'static str {
    return "123";
}

pub fn create() -> bool {
    return false
}

pub fn check(id: &str) -> bool {
    return false
}

pub fn summary() {
}
