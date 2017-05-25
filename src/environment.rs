use std::path::PathBuf;
use std::fmt;
use std;
use slug;

use std::sync::{Arc, Mutex, Once, ONCE_INIT};
use std::time::Duration;
use std::{mem, thread};

use clap::{ArgMatches};
use db::{Database as DB, preferences};

use project::{active_project};

use utils::{make_absolute, if_occurred, print_help};

pub enum HostEnv {
    Cluster,
    Metal
}

pub enum AppEnv {
    Production,
    Development
}

pub enum Vault {
    Developer,
    Admin
}

impl fmt::Display for Vault {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        vault_name(*self);
        Ok(())
    }
}

pub struct Environment {
    pub vault: Vault,
    pub root:  PathBuf,
    pub host:  HostEnv,
    pub env:   AppEnv,
}

#[derive(Clone)]
struct EnvironmentSingleton {
    inner: Arc<Mutex<Environment>>,
}

// Initialize it to a null value
static mut SINGLETON: *const EnvironmentSingleton = 0 as *const EnvironmentSingleton;
static ONCE: Once = ONCE_INIT;

pub fn initialize_environment(m: &ArgMatches) {

    // Initialize the barge root with an order of precedence
    let mut root : PathBuf = match m.value_of("BARGE_ROOT") {
        Some(path) => PathBuf::from(make_absolute(path)),
        None => {
            let prefs = preferences();
            std::env::current_dir().unwrap()
        }
    };

    // Handle dynamic barge root based on new project creation
    if let Some(matches) = m.subcommand_matches("projects") {
        match matches.occurrences_of("create") {
            1 => {
                let project_name = &slug::slugify(
                    matches.value_of("create").unwrap())[..];

                // update root with a new directory based on
                // creating a new project in the current dir.
                root = PathBuf::from(format!(
                    "{}/{}",
                    std::env::current_dir().unwrap().to_str().unwrap(),
                    project_name));
            },
            _ => {}
        }
    }

    let host  = HostEnv::Metal;
    let env   = AppEnv::Development;
    let vault = Vault::Developer;

    let env = Environment {
        vault: vault,
        root: root,
        host: host,
        env:  env
    };

    unsafe {
        ONCE.call_once(|| {
            // Make it
            let singleton = EnvironmentSingleton {
                inner: Arc::new(Mutex::new(env))
            };

            // Put it in the heap so it can outlive this call
            SINGLETON = mem::transmute(Box::new(singleton));
        });
    }
}

pub fn singleton() -> EnvironmentSingleton {
    unsafe {
        // Now we give out a copy of the data that is safe to use concurrently.
        (*SINGLETON).clone()
    }
}

impl Environment {
    pub fn root_str<'a>(&self) -> &'a str{
        let r = self.root.clone().to_str();
        match r {
            Some(str) => { str },
            None => { panic!("Could not convert barge_root to &str") }
        }
    }

    pub fn global() -> Environment {
        *singleton().inner.lock().unwrap()
    }
}


/////////////////////////////////////////////////
// Private

fn vault_name<'a>(v: Vault) -> &'a str {
    match v {
        Developer => "developer",
        Admin => "admin"
    }
}