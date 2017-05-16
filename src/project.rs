#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectData {
    pub barge_root: String,
    pub vault_root: String,
    pub name: String,
}

pub fn list() -> Vec<ProjectData> {
    return Vec::new();
}

pub fn create() {
}
