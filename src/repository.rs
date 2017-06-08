use std::path::PathBuf;
use environment::{Environment};

#[derive(Debug)]
pub struct Repository {
    pub name: String,
    pub path: String,
    pub git: String
}

const DEFAULT_REPO_BASE: &'static str = "git@github.com:";

impl Repository {
    pub fn new(path: &str, git: &str) -> Repository {
        let env = Environment::global();
        let name = path.to_string();
        let full_path;

        if path.starts_with('/') {
            full_path = path.to_string();
        } else {
            full_path = format!("{}/{}", env.root.to_str().unwrap(), path);
        }

        let git_full = match path.find('@') {
            Some(_) => path.to_string(),
            None => {
                format!("{}{}", DEFAULT_REPO_BASE, git)
            }
        };

        Repository {
            name: name,
            path: full_path,
            git: git_full,
        }
    }
}

pub fn attach_vault(env: &Environment) -> Repository {
    let barge_root = env.root_str().to_string();
    let path = format!("{}/vault/{}", barge_root, env.vault.to_str());
    let git = format!("crowdist/vault-{}", env.vault.to_str());

    new_repo(&path, &git)
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
