#![feature(plugin)]

// extern crate commando;
extern crate commando;

#[cfg(test)]
mod test {
    // use commando::{App};

    #[test]
    fn creation() {
        let app = super::commando::app::App::new("commando");
    }
}
