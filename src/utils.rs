use std;
use std::path::PathBuf;

use termion;
use termion::color;
use termion::event::Key;
use termion::input::TermRead;

use clap::{ArgMatches};
use cli::tree::{build_tree as tree};

pub fn green() -> termion::color::Fg<termion::color::Rgb> {
    color::Fg(color::Rgb(50, 205, 50))
}

pub fn red() -> termion::color::Fg<termion::color::Rgb> {
    color::Fg(color::Rgb(255, 0, 0))
}

pub fn reset() -> termion::style::Reset {
    termion::style::Reset
}

pub fn exit() -> bool {
    std::process::exit(0);
    true
}

pub fn print_help() {
    tree().print_help();
    exit();
}

pub fn make_absolute(path: &str) -> String {
    if path.find('/').unwrap() == 0 {
        debug!("detected absolute path: {}", path);
        return path.to_string();
    } else {
        let current = std::env::current_dir().unwrap();
        let str : String = format!("{}/{}", current.to_str().unwrap(), path);
        return str;
    }
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
    return false;
}
