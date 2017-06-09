// use std::path::PathBuf;
use environment::{Environment};
use utils::{make_absolute};

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

    pub fn load_manifest() -> Vec<Repository> {
        let repos: Vec<Repository> = Vec::new();
        let env = Environment::global();

        let m = match env.args.occurrences_of("manifest") {
            0 => {
                panic!("Manifest not provided during project creation");
            },
            _ => {
                let file = env.args.value_of("manifest").unwrap();
                let abs = make_absolute(file);
                println!("file: {}", abs);
                file
            }
        };

        println!("manifest: {:?}", m);
        repos
    }
}

pub fn attach_vault(env: &Environment) -> Repository {
    let barge_root = env.root_str().to_string();
    let path = format!("{}/vault/{}", barge_root, env.vault.to_str());
    let git = format!("crowdist/vault-{}", env.vault.to_str());

    new_repo(&path, &git)
}

pub fn new_repo(path: &String, git: &String) -> Repository {
    Repository::new(&path[..], &git[..])
}
