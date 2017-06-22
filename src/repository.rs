use std::path::PathBuf;
use environment::Environment;
use utils::make_absolute;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::collections::{HashMap, BTreeMap};
use serde_yaml;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum RepoClass {
    Lib,
    System,
    Vault,
    Service,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Repository {
    pub class: RepoClass,
    pub name: String,
    pub path: String,
    pub git: String,
}

const DEFAULT_REPO_BASE: &'static str = "git@bitbucket.org:";

impl Repository {
    pub fn new(path: String, git: String, class: RepoClass, name: String) -> Repository {
        let env = Environment::global();
        // let name = path.to_string();
        let full_path;

        if path.starts_with('/') {
            full_path = path.to_string();
        } else {
            full_path = format!("{}/{}", env.root.to_str().unwrap(), path);
        }

        let git_full = match path.find('@') {
            Some(_) => path.to_string(),
            None => format!("{}{}", DEFAULT_REPO_BASE, git),
        };

        Repository {
            name: name,
            path: full_path,
            git: git_full,
            class: class,
        }
    }

    pub fn load_manifest() -> Vec<Repository> {
        let mut repos: Vec<Repository> = Vec::new();
        let env = Environment::global();

        match env.args.occurrences_of("manifest") {
            0 => {
                panic!("Manifest not provided during project creation");
            }
            _ => {
                let file = env.args.value_of("manifest").unwrap();
                let manifest = make_absolute(file);

                if !PathBuf::from(&manifest).exists() {
                    println!("manifest file: {}", &manifest);
                    println!("manifest file raw: {}", &file);
                    panic!("Manifest could not be located at given path");
                }

                let f = File::open(manifest).unwrap();
                let mut buf_reader = BufReader::new(f);
                let mut contents = String::new();

                buf_reader.read_to_string(&mut contents).unwrap();

                let s: BTreeMap<String, Vec<HashMap<String, String>>> =
                    serde_yaml::from_str(&contents[..]).unwrap();

                for (typ, entries) in &s {
                    let class: RepoClass = RepoClass::from_str(&typ);

                    for entry in entries {
                        // println!("type: {}, entry: {:?}", typ, entry);

                        let git = entry.get("git").unwrap();
                        let name = match entry.get("name") {
                            None => typ,
                            Some(n) => n,
                        };

                        let mut dir_name = name.clone();

                        if class == RepoClass::Vault {
                            dir_name = entry.get("type").unwrap().to_string();
                        }

                        let prefix = class.to_string();

                        let path =
                            format!("{}/{}/{}", env.root.to_str().unwrap(), prefix, dir_name);

                        repos.push(Repository::new(
                            path.to_string(),
                            git.to_string(),
                            class.clone(),
                            name.to_string(),
                        ));
                    }

                }
                file
            }
        };

        repos
    }
}

impl RepoClass {
    pub fn from_str(s: &str) -> RepoClass {
        match s {
            "lib" => RepoClass::Lib,
            "system" => RepoClass::System,
            "services" => RepoClass::Service,
            "vault" => RepoClass::Vault,
            _ => RepoClass::Service,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            &RepoClass::Lib => "lib",
            &RepoClass::System => "system",
            &RepoClass::Service => "services",
            &RepoClass::Vault => "vault",
        }.to_string()
    }
}
