use std;
use std::path::PathBuf;

use termion;
use termion::color;
// use termion::event::Key;
// use termion::input::TermRead;

use clap::ArgMatches;
use cli::tree::build_tree as tree;

use regex::Regex;

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
    println!("\n");
    std::process::exit(0);
    true
}

pub fn print_help() {
    let help = tree().print_help();
    debug!("print_help: {:?}", help);
    exit();
}

pub fn make_absolute(path: &str) -> String {
    let current = std::env::current_dir().unwrap();
    make_absolute_from_root(path, current.to_str().unwrap())
}

pub fn make_absolute_from_root(p: &str, prefix: &str) -> String {
    let home = std::env::home_dir().unwrap();
    let mut path = p.to_string();

    let r1 = Regex::new(r"^./").unwrap();
    let r2 = Regex::new(r"^~").unwrap();

    path = r1.replace_all(&path[..], "").to_string();
    path = r2.replace_all(&path[..], home.to_str().unwrap())
        .to_string();

    let path_starts_with_slash = match path.find('/') {
        Some(i) => i < 1,
        None => false,
    };

    if path_starts_with_slash {
        path
    } else {
        format!("{}/{}", prefix, path)
    }
}

pub fn path_with_subpath(root: &PathBuf, subdir: &str) -> String {
    format!("{}/{}", root.to_str().unwrap(), subdir)
}

pub fn if_occurred<F>(name: &str, matches: &ArgMatches, func: F) -> bool
where
    F: Fn() -> bool,
{
    match matches.occurrences_of(name) {
        1 => func() && exit(),
        0 => false,
        _ => false,
    }
}

pub fn check_path_exists(path: &PathBuf) -> bool {
    debug!("check_path_exists: {}", &path.to_str().unwrap());
    return false;
}
