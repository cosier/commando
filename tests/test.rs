#![feature(plugin)]

// extern crate commando;
extern crate commando;

#[cfg(test)]
mod test {
    #[test]
    fn creation() {
        super::commando::app::App::new("commando");
    }
}
