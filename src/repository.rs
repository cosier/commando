use std::path::PathBuf;
use environment::{Environment};

pub struct Repository {
    pub path: String,
    pub git: String
}

impl Repository {
    pub fn new(path: &str, git: &str) -> Repository {
        Repository {
            path: path.to_string(),
            git: git.to_string()
        }
    }
}

pub fn attach_vault(env: &Environment) -> Repository {
    let barge_root = env.root_str().to_string();
    new_repo(
        &barge_root,
        &format!("{}/vault/{}", barge_root, env.vault))
}

pub fn service_repositories(env: &Environment) -> Vec<Repository> {
    let mut repos: Vec<Repository> = Vec::new();
    let services = ["core", "doorman", "redis", "client"];

    for service in services.into_iter() {
        repos.push(new_service_repo(&env.root, service));
    }

    repos
}

pub fn new_repo(path: &String, git: &String) -> Repository {
    Repository::new(&path[..], &git[..])
}


/////////////////////////////////////////////////
// Private

fn new_service_repo(root: &PathBuf, name: &str) -> Repository {
    let root_str = root.to_str().unwrap();
    let service_dir = format!("{}/services/{}", root_str, &name.clone());
    let repo_addr = format!("crowdist/{}", &name.clone());

    new_repo(&service_dir, &repo_addr)
}
