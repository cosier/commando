use std::cell::{RefCell};
use std::path::PathBuf;
use std::fs;

use git2;
use git2::build::{RepoBuilder, CheckoutBuilder};
use git2::{RemoteCallbacks, Progress, FetchOptions};
use std::env;

use utils::errors::{CommandoResult};

struct State {
    progress: Option<Progress<'static>>,
    total: usize,
    current: usize,
    path: Option<PathBuf>,
    newline: bool,
}

fn print(state: &mut State) {
    let stats = state.progress.as_ref().unwrap();
    let network_pct = (100 * stats.received_objects()) / stats.total_objects();
    // let index_pct = (100 * stats.indexed_objects()) / stats.total_objects();
    // let co_pct = if state.total > 0 {
    //     (100 * state.current) / state.total
    // } else {
    //     0
    // };
    let kbytes = stats.received_bytes() / 1024;
    if stats.received_objects() == stats.total_objects() {
        if !state.newline {
            println!("");
            state.newline = true;
        }
        // print!("Resolving deltas {}/{}\r", stats.indexed_deltas(),
        //        stats.total_deltas());
    } else {
        print!("Loaded {:3}% ({:4} kb) {}\r",
               network_pct, kbytes, stats.received_objects());
    }
}


pub fn fetch(url: &str, dest: &str) -> CommandoResult<git2::Repository> {
    let path = PathBuf::from(dest);
    // Create a local anonymous remote in the repository to fetch the url

    let state = RefCell::new(State {
        progress: None,
        total: 0,
        current: 0,
        path: None,
        newline: false,
    });

    Ok(clone(state, url, path))
}

fn clone(state: RefCell<State>, url: &str, path: PathBuf) -> git2::Repository {
    let mut co = CheckoutBuilder::new();
    let mut cb = RemoteCallbacks::new();
    let mut fo = FetchOptions::new();

    cb.transfer_progress(|stats| {
        let mut state = state.borrow_mut();
        state.progress = Some(stats.to_owned());
        print(&mut *state);
        true
    });

    co.progress(|path, cur, total| {
        let mut state = state.borrow_mut();
        state.path = path.map(|p| p.to_path_buf());
        state.current = cur;
        state.total = total;
        print(&mut *state);
    });

    cb.credentials(|_, _, _| creds());
    fo.remote_callbacks(cb);

    match git2::Repository::init(&path) {
        Ok(repo) => repo,

        Err(err) => {
            panic!("Git Fetch failed to create repository because {}", err)
        }
    };


    if PathBuf::from(&path).exists() {
        fs::remove_dir_all(&path).unwrap();
    }

    let mut builder = RepoBuilder::new();
    builder.fetch_options(fo).with_checkout(co);

    // debug!("CLONING: {}", &url);
    println!("\nCloning: {}", &url);

    builder.clone(url, &path).unwrap()
}

fn creds() -> Result<git2::Cred, git2::Error>{
    let home = env::home_dir().unwrap();
    let hstr = home.to_str().unwrap();
    let public = PathBuf::from(format!("{}/.ssh/id_rsa.pub", hstr));
    let private = PathBuf::from(format!("{}/.ssh/id_rsa", hstr));

    let k = git2::Cred::ssh_key("git", Some(&public.as_path()), &private.as_path(), None);
    return k;
}
