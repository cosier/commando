
use std::sync::{Arc, Mutex, Once, ONCE_INIT};
use std::path::PathBuf;
use std::{fmt, mem};
use std;

use slug;
use clap::ArgMatches;
use persistence::{Persistence};
use utils::make_absolute;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum HostEnv {
    Cluster,
    Metal,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AppEnv {
    Production,
    Development,
}


#[derive(Clone, Debug)]
pub enum Vault {
    Development,
    Secure,
}

impl fmt::Display for Vault {
    fn fmt(&self, _: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        vault_name(self);
        Ok(())
    }
}
impl Vault {
    pub fn to_str(&self) -> &str {
        vault_name(self)
    }
}

#[derive(Clone)]
pub struct Environment<'a> {
    pub vault: Vault,
    pub root: PathBuf,
    pub host: HostEnv,
    pub env: AppEnv,
    pub project_name: String,
    pub args: ArgMatches<'a>,
}

#[derive(Clone)]
pub struct EnvironmentSingleton<'a> {
    pub inner: Arc<Mutex<Environment<'a>>>,
}

// Initialize it to a null value
static mut SINGLETON: *const EnvironmentSingleton = 0 as *const EnvironmentSingleton;
static ONCE: Once = ONCE_INIT;

pub fn initialize_environment(m: &ArgMatches) {
    let mut project_name: String = "app".to_string();
    let mut root_is_explicit = false;

    // Initialize the barge root with an order of precedence
    let mut root: PathBuf = match m.occurrences_of("barge-root") {
        1 => {
            let path = m.value_of("barge-root").unwrap();
            root_is_explicit = true;
            PathBuf::from(make_absolute(path))
        }
        _ => {
            let prefs = Persistence::new().preferences;
            if let Some(active_project) = prefs.active_project {
                let barge_root = prefs
                    .projects
                    .get(&active_project)
                    .unwrap()
                    .barge_root
                    .clone();
                PathBuf::from(barge_root)
            } else {
                std::env::current_dir().unwrap()
            }
        }
    };

    // Handle dynamic barge root based on new project creation
    if let Some(matches) = m.subcommand_matches("projects") {
        match matches.occurrences_of("create") {
            // BRANCH: fresh barge
            1 => {
                project_name = slug::slugify(matches.value_of("create").unwrap());

                // update root with a new directory based on
                // creating a new project in the current dir.
                if !root_is_explicit {
                    root = PathBuf::from(format!(
                        "{}/{}",
                        std::env::current_dir().unwrap().to_str().unwrap(),
                        &project_name[..]
                    ));
                }
            }

            // BRANCH: Attempt to load an existing active project prefs
            _ => {}
        }
    }

    let host = HostEnv::Metal;
    let env = AppEnv::Development;

    let vault = match m.value_of("vault") {
        None => Vault::Development,
        Some(str) => {
            match str {
                "dev" => Vault::Development,
                "development" => Vault::Development,
                "secure" => Vault::Secure,
                &_ => {
                    println!("valid vault options: development, secure\n");
                    panic!(format!("Invalid vault parameter: {}", str))
                }
            }
        }
    };

    let env = Environment {
        project_name: project_name,
        vault: vault,
        root: root,
        host: host,
        env: env,
        args: m.clone(),
    };

    unsafe {
        ONCE.call_once(|| {
            // Make it
            let singleton = EnvironmentSingleton { inner: Arc::new(Mutex::new(env)) };

            // Put it in the heap so it can outlive this call
            SINGLETON = mem::transmute(Box::new(singleton));
        });
    }
}

pub fn singleton() -> EnvironmentSingleton<'static> {
    unsafe {
        // Now we give out a copy of the data that is safe to use concurrently.
        (*SINGLETON).clone()
    }
}

impl<'a> Environment<'a> {
    pub fn root_str(&self) -> &str {
        self.root.to_str().unwrap()
    }

    pub fn global() -> Environment<'static> {
        let inner: &Arc<Mutex<Environment<'static>>> = &singleton().inner;
        let copy = inner.lock().unwrap().clone();
        copy
    }
}


/////////////////////////////////////////////////
// Private

fn vault_name<'a>(v: &Vault) -> &'a str {
    match v {
        &Vault::Development => "dev",
        &Vault::Secure => "secure",
    }
}
