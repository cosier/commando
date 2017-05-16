use db::Database as DB;

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

pub fn list() -> Vec<ProjectData> {
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
