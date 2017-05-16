#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectData {
    pub barge_root: String,
    pub vault_root: String,
    pub name: String,
}
