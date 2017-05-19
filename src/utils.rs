use std;

use termion;
use termion::color;
use termion::event::Key;
use termion::input::TermRead;

pub fn green() -> termion::color::Fg<termion::color::Rgb> {
    color::Fg(color::Rgb(50, 205, 50))
}

pub fn red() -> termion::color::Fg<termion::color::Rgb> {
    color::Fg(color::Rgb(255, 0, 0))
}

pub fn reset() -> termion::style::Reset {
    termion::style::Reset
}

pub fn exit(msg: &str) {
    error!("Exiting -> {}", msg);
    std::process::exit(0);
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
