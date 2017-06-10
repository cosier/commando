use std::path::PathBuf;
use environment::{Environment};
use utils::{make_absolute};
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::collections::{HashMap, BTreeMap};
use serde_yaml;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RepoClass {
    Lib, System, Vault, Service
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Repository {
    pub class: RepoClass,
    pub name: String,
    pub path: String,
    pub git: String,
}

const DEFAULT_REPO_BASE: &'static str = "git@github.com:";

impl Repository {
    pub fn new(path: &str, git: &str, class: RepoClass) -> Repository {
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
            class: class
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
                let manifest = make_absolute(file);

                if !PathBuf::from(&manifest).exists() {
                    println!("manifest file: {}", &manifest);
                    panic!("Manifest could not be located at given path");
                }

                let f = File::open(manifest).unwrap();
                let mut buf_reader = BufReader::new(f);
                let mut contents = String::new();

                buf_reader.read_to_string(&mut contents).unwrap();

                let s: BTreeMap<String, Vec<HashMap<String, String>>> =
                    serde_yaml::from_str(&contents[..]).unwrap();

                for (typ, entry) in &s {
                    println!("type: {}, entry: {:?}", typ, entry);
                }
                // println!("manifest: {:?}\n", s);
                file
            }
        };

        println!("manifest: {:?}", m);
        repos
    }
}

// pub fn attach_vault(env: &Environment) -> Repository {
//     let barge_root = env.root_str().to_string();
//     let path = format!("{}/vault/{}", barge_root, env.vault.to_str());
//     let git = format!("crowdist/vault-{}", env.vault.to_str());

//     new_repo(&path, &git)
// }

// pub fn new_repo(path: &String, git: &String) -> Repository {
//     Repository::new(&path[..], &git[..])
// }
