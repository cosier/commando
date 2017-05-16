
use termion;
use termion::color;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub fn green() -> termion::color::Fg<termion::color::Rgb> {
    color::Fg(color::Rgb(50, 205, 50))
}

pub fn red() -> termion::color::Fg<termion::color::Rgb> {
    color::Fg(color::Rgb(255, 0, 0))
}

pub fn reset() -> termion::style::Reset {
    termion::style::Reset
}
