// use cursive::Cursive;
// use cursive::views::{Dialog, TextView};


extern crate jfs;
use jfs::Store;
use std::path::{PathBuf};
use std::fs::{File};
use std::io;
use std::io::prelude::*;
use std::collections::{HashMap};
use std;

use log::LogLevel;
use clap::{Arg, App as CliApp, SubCommand, ArgMatches};

use project::ProjectData;
use preferences::Preferences;

pub struct App<'a> {
    name: &'a str,
    db: Store,
    preferences: Preferences
}


impl<'a> App<'a> {
    pub fn new(name: &str) -> App {
        let mut db = App::db(name);
        let prefs = App::preferences(name, &mut db);

        {
            let app = App {
                preferences: prefs,
                name: name,
                db: db,
            };

            app.initialize();
            app
        }
    }

    fn db(name: &str) -> Store {
        let mut cfg = jfs::Config::default();
        cfg.pretty = true;
        cfg.single = true;
        cfg.indent = 4;

        return Store::new_with_cfg(name, cfg).unwrap()
    }

    fn preferences(name: &str, db: &mut Store) -> Preferences {
        let mut db_file = String::from(
            std::env::home_dir().unwrap().to_owned().to_str().unwrap());

        db_file.push_str(&format!("/.{}.json", name)[..]);
        db.set_path(PathBuf::from(&db_file[..]));

        return match db.get::<Preferences>(name) {
            Ok(d) => d,
            Err(e) => {
                println!("Preferences: Not loaded from disk - {}", e);
                let prefs = Preferences {
                    active_project: None,
                    projects: HashMap::new()
                };

                db.save_with_id(&prefs, name).unwrap();
                return prefs
            }
        };
    }

    fn initialize(&self) {
        if log_enabled!(LogLevel::Debug) {
            &self.dump();
        }
    }

    fn dump(&self) {
        let db_file = &self.db.get_db_path();
        match cat(db_file) {
            Ok(s) => debug!("--------------\ndb:\n{}\n",s),
            Err(why) => error!("Could not read commando db: {} \npath: {}",
                               why, db_file.to_str().unwrap())
        };
    }

    fn matches(&self) -> ArgMatches {
        return CliApp::new("Commando")
            .version("0.1-alpha")
            .author("Bailey Cosier. <bailey@crowdist.com>")
            .about("Swiss army knife management plane")

            .arg(Arg::with_name("project")
                 .short("p")
                 .long("project")
                 .value_name("PROJECT_NAME")
                 .help("Specify project explicitly")
                 .takes_value(true))

            // GUI
            .subcommand(SubCommand::with_name("gui")
                        .about("open a dynamic ui based on ncurses")
                        .arg(Arg::with_name("debug")
                             .short("d")
                             .help("print debug information verbosely")))

            // ProjectDatas
            .subcommand(SubCommand::with_name("projects")
                        .about("manage projects within commando")
                        .arg(Arg::with_name("list")
                             .short("l")
                             .long("list")
                             .help("List available Projects"))
                        .arg(Arg::with_name("create")
                            .short("c")
                             .long("create")
                            .help("Create a new Project")))

            // Deploy
            .subcommand(SubCommand::with_name("deploy")
                        .about("deployment")
                        .arg(Arg::with_name("debug")
                             .short("d")
                             .help("print debug information verbosely")))

            // Monitor
            .subcommand(SubCommand::with_name("monitor")
                        .about("status")
                        .arg(Arg::with_name("debug")
                             .short("d")
                             .help("live status")))

            // Logs
            .subcommand(SubCommand::with_name("logs")
                        .about("status")
                        .arg(Arg::with_name("debug")
                             .short("d")
                             .help("print debug information verbosely")))

            // Utiltiies
            .subcommand(SubCommand::with_name("utils")
                        .about("utility belt")
                        .arg(Arg::with_name("compile")
                             .short("c")
                             .long("compile")
                             .help("config variable compiler")))

            .get_matches();
    }

    pub fn cli(&self)  {
        let matches: &ArgMatches = &self.matches();
        let px:&str= matches.value_of("project").unwrap_or("none");
        info!("px: {}", px);
    }
}

fn cat(path: &PathBuf) -> io::Result<String> {
    debug!("\n-----\ndump: {}", &path.to_str().unwrap());

    let mut f = try!(File::open(&path));
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
