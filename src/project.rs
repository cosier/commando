use db::Database as DB;

use std;
use std::{fmt, fs};
use std::env::current_dir;
use std::path::PathBuf;
use std::fs::{create_dir};
use utils::{exit, check_path_exists, make_absolute_from_root, print_red, print_green};
use repository::{Repository, new_repo, attach_vault, service_repositories};
use environment::{Environment};
// use git2::{Repository as GitRepository};
use db::{preferences};
use git;

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

impl fmt::Display for ProjectData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[project:{}:{}]", self.name, self.barge_root)
    }
}

fn assert_project_exists(name: &str) {
}

pub fn list() -> Vec<Box<ProjectData>> {
    let projects = DB::list_projects();
    projects
}

pub fn active_project() -> Option<String> {
    preferences().active_project
}

/// Creates a Project model definition and directory structure
pub fn create_project(name: &str, path: PathBuf) -> bool {

    if check_path_exists(&path) {
        fs::remove_dir_all(&path).unwrap();

        // println!("Path Error: Found existing directory:\n{}",
        //          path.to_str().unwrap());
        // exit();
    }

    if check_project_exists(name) {
        println!("Existing Project Error: Cannot create project with name:\n{}",
                 path.to_str().unwrap());
        exit();
    }

    let env = Environment::global();

    create_barge(&env) &&
        initialize_barge(&env)
}

pub fn promote_project(name: &str) -> bool {
    true
}

pub fn purge_project(name: &str) -> bool {
    true
}

pub fn info_project(name: &str) -> bool {
    true
}

pub fn setup_project(name: &str) -> bool {
    true
}

/////////////////////////////////////////////////
// Private

fn check_project_exists(name: &str) -> bool {
   false
}

fn create_barge(env: &Environment) -> bool {
    create_folder(&env.root, None)
}

/// Checks sub directories for code repository setup
fn initialize_barge(env: &Environment) -> bool {

    let paths = [
        "lib",
        "services",
        "system",
        "vault",
    ];

    let mut repositories = vec![
        Repository::new("lib/bash", "crowdist/libbash"),
        Repository::new("lib/lua", "crowdist/liblua"),
        Repository::new("system/os", "crowdist/os"),
        attach_vault(env),
    ];

    for service in service_repositories(env) {
        repositories.push(service);
    }

    let barge_root = &env.root.to_str().unwrap();
    println!("Barge initialization @ \n{}\n", &barge_root);

    for p in paths.into_iter() {
        if !create_folder(&env.root, Some(p)) {
            exit();
        } else {
            let mut success = false;
            let root = &env.root.to_str().unwrap();
            let abs_path = make_absolute_from_root(p, root);
            let mut msgs = Vec::new();

            for repo in &repositories {
                if repo.path[..].contains(&abs_path) {
                    msgs.push(format!("  -  git: {}", &repo.git));
                    success = true;

                    let repo_url = format!("{}{}", "crowdist/", repo.path.clone());
                    let clone = match git::fetch(&repo.git, &repo.path) {
                        Err(e) => panic!("failed to clone: {}", e),
                        Ok(r) => {
                            msgs.push(format!(" -  cloned: {:?} to {:?}", &repo.git, &repo.path));
                            r
                        },
                    };

                }
            }

            if success {
                println!("\n‣ {} - ✓", p);
            } else {
                print_red(format!("‣ {} - ✗", p));
            }

            for msg in msgs {
                println!("{}", msg);
            }
        }
    }


    print_green("\nBarge project creation done.\n".to_string());
    true
}

/// Create folder based on a root path and relative subpath
fn create_folder(root: &PathBuf, subpath: Option<&str>) -> bool {
    let path: PathBuf = match subpath {
        None => root.clone(),
        Some(p) => {
            let combo: PathBuf = PathBuf::from(
                format!("{}/{}",
                        root.to_str().unwrap(),
                        p));

            combo.clone()

        },
    };

    if path.exists() {
        return true;
    }

    debug!("creating: {}", path.to_str().unwrap());
    match create_dir(path.clone()) {
        Ok(_) => true,
        Err(e) => {
            error!("Could not create directory: {}\n because {}",
                   path.to_str().unwrap(),
                   e);
            exit();
            false
        }
    }
}
