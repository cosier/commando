use std::collections::{HashMap};
use project::ProjectData;

#[derive(Serialize, Deserialize, Debug)]
pub struct Preferences {
    pub active_project: Option<String>,
    pub projects: HashMap<String,ProjectData>
}
