use std::path::PathBuf;
use environment::{Environment};

pub struct Repository<'a> {
    pub path: &'a str,
    pub git: &'a str
}

impl<'a> Repository<'a> {
}

pub fn attach_vault<'a>(env: &Environment) -> Repository<'a> {
    let barge_root = env.root_str();

    new_repo(
        barge_root,
        &format!("{}/vault/{}", barge_root, env.vault)[..]
    )
}

pub fn service_repositories<'a>(env: &Environment) -> Vec<Repository<'a>> {
    let repos: Vec<Repository> = Vec::new();
    let services = ["core", "doorman", "redis", "client"];

    for service in services.into_iter() {
        repos.push(new_service_repo(&env.root, service));
    }

    repos
}

pub fn new_repo<'a>(path: &'a str, git: &'a str) -> Repository<'a> {
    Repository { path: path, git: git }
}


/////////////////////////////////////////////////
// Private

fn new_service_repo<'a>(root: &PathBuf, name: &str) -> Repository<'a> {
    new_repo(
        &format!("{}/services/{}", root.to_str().unwrap(), name)[..],
        &format!("crowdist/{}", name)[..]
    )
}
