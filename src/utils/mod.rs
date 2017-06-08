use std;
use std::path::PathBuf;

use termion;
use termion::color;
// use termion::event::Key;
// use termion::input::TermRead;

use clap::{ArgMatches};
use cli::tree::{build_tree as tree};

pub mod errors;

pub fn green() -> termion::color::Fg<termion::color::Rgb> {
    color::Fg(color::Rgb(50, 205, 50))
}

pub fn red() -> termion::color::Fg<termion::color::Rgb> {
    color::Fg(color::Rgb(255, 0, 0))
}

pub fn print_red(input: String) {
    println!("{}{}{}", red(), &input[..], reset());
}

pub fn print_green(input: String) {
    println!("{}{}{}", green(), &input[..], reset());
}

pub fn reset() -> termion::style::Reset {
    termion::style::Reset
}

#[allow(unreachable_code)]
pub fn exit() -> bool {
    std::process::exit(0);
    true
}

pub fn print_help() {
    let help = tree().print_help();
    debug!("print_help: {:?}", help);
    exit();
}

pub fn make_absolute(path: &str) -> String {
    let r = path.find('/');
    let indexed = match r {
        Some(_) => true,
        None => false
    };

    if indexed {
        debug!("detected absolute path: {}", path);
        return path.to_string();
    } else {
        let current = std::env::current_dir().unwrap();
        let str : String = format!("{}/{}", current.to_str().unwrap(), path);
        return str;
    }
}

pub fn make_absolute_from_root(path: &str, root: &str) -> String {
    let r = path.find('/');
    let indexed = match r {
        Some(_) => true,
        None => false
    };

    if indexed {
        debug!("detected absolute path: {}", path);
        return path.to_string();
    } else {
        let str : String = format!("{}/{}", root, path);
        return str;
    }
}

pub fn path_with_subpath(root: &PathBuf, subdir: &str) -> String {
    format!("{}/{}", root.to_str().unwrap(), subdir)
}

pub fn if_occurred<F>(name: &str, matches: &ArgMatches, func: F) -> bool where F: Fn() -> bool {
    match matches.occurrences_of(name) {
        1 => {
            func() && exit()
        },
        0 => { false },
        _ => { false }
    }
}

pub fn check_path_exists(path: &PathBuf) -> bool {
    debug!("check_path_exists: {}", &path.to_str().unwrap());
    return false;
}
