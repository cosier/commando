use db::Database as DB;

use std::{fmt, fs};
use std::path::PathBuf;
use std::fs::{create_dir};
use utils::{exit, check_path_exists, make_absolute_from_root, print_red, print_green};

use repository::{Repository};
use environment::{Environment, AppEnv, HostEnv};

use db::{Database};
use git;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectData {
    pub barge_root: String,
    pub vault_root: String,
    pub env: AppEnv,
    pub host: HostEnv,
    pub name: String,
    pub repos: Vec<Repository>
}

impl fmt::Display for ProjectData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[project:{}:{}]", self.name, self.barge_root)
    }
}

pub fn list() -> Vec<Box<ProjectData>> {
    let projects = DB::list_projects();
    projects
}

pub fn active_project() -> Option<String> {
    DB::prefs().active_project
}

/// Creates a Project model definition and directory structure
pub fn create_project(name: &str, path: PathBuf) -> bool {
    debug!("create_project: {}", name);

    if check_path_exists(&path) {
        fs::remove_dir_all(&path).unwrap();
    }

    if check_project_exists(name) {
        debug!("Existing Project Error: Cannot create project with name:\n{}",
                 path.to_str().unwrap());
        exit();
    }

    let env = Environment::global();

    create_barge(&env) &&
        initialize_barge(&env)
}

pub fn promote_project(name: &str) -> bool {
    debug!("promote_project: {}", name);
    true
}

pub fn purge_project(name: &str) -> bool {
    debug!("purge_project: {}", name);
    true
}

pub fn info_project(name: &str) -> bool {
    debug!("info_project: {}", name);
    true
}

pub fn setup_project(name: &str) -> bool {
    debug!("setup_project: {}", name);
    true
}

pub fn list_projects() -> bool {
    let p = Database::prefs().projects;
    for (name, _) in p.iter() {
       println!("project: {}", name);
    }

    if p.iter().len() < 1 {
        println!("No projects found");
    }

    true
}

/////////////////////////////////////////////////
// Private

fn check_project_exists(name: &str) -> bool {
    debug!("check_project_exists: {}", name);
    false
}

fn create_barge(env: &Environment) -> bool {
    create_folder(&env.root, None)
}

/// Checks sub directories for code repository setup
fn initialize_barge(env: &Environment) -> bool {
    let repositories: Vec<Repository> = Repository::load_manifest();
    let barge_root = &env.root.to_str().unwrap();
    let paths = [
        "lib",
        "services",
        "system",
        "vault",
    ];

    debug!("Barge initialization @ \n{}\n", &barge_root);

    for p in paths.into_iter() {
        if !create_folder(&env.root, Some(p)) {
            exit();
        } else {
            let mut success = false;
            let root = &env.root.to_str().unwrap();
            let mut msgs = Vec::new();

            for repo in &repositories {
                let abs_path = make_absolute_from_root(p, root);
                if repo.path[..].contains(&abs_path) {
                    msgs.push(format!("  -  git: {}", &repo.git));
                    success = true;
                    match git::fetch(&repo.git, &repo.path) {
                        Err(e) => panic!("failed to clone: {}", e),
                        Ok(r) => r,
                    };
                } else {
                    error!("mismatch: {} vs {}", repo.path, &abs_path);
                }
            }

            if success {
                println!("\n‣ {} - ✓", p);
            } else {
                print_red(format!("‣ {} - ✗", p));
                println!("{:?}", &repositories);
            }

            for msg in msgs {
                debug!("{}", msg);
            }
        }
    }

    let mut prefs = Database::prefs();
    let name = env.project_name.clone();

    let project = ProjectData {
        name: name.clone(),
        barge_root: env.root.to_str().unwrap().to_string(),
        vault_root: env.vault.to_str().to_string(),
        host: HostEnv::Metal,
        env: AppEnv::Development,
        repos: repositories
    };

    prefs.projects.insert(name.clone(), project);
    prefs.active_project = Some(name);

    DB::save(prefs);

    print_green("\nBarge project created successfully.\n".to_string());
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
