use clap::{Arg, App as CliApp, SubCommand, ArgMatches};
use db::Database as DB;

use std::fmt;
use std::any::Any;
use std::io::{Read, Write, stdout, stdin};

use project::{ProjectData};
use utils::{green, red, reset};

pub fn parse() {
    let root_matches: ArgMatches = initiate_matches();
    let project_id = root_matches.value_of("project").unwrap_or("none");
    // info!("project_id: {}", project_id);

    // List Projects
    if let Some(matches) = root_matches.subcommand_matches("projects") {
        info!("projects subcommand activiated");

        match matches.occurrences_of("list") {
            1  => display_as_list("Projects", DB::list_projects()),
            _ => ()
        }

        match matches.occurrences_of("create") {
            1  => println!("creating project"),
            _  => ()
        }
    }
}

fn display_as_list(title: &str, mut collection: Vec<Box<ProjectData>>) {
    println!("\n{}----------- {} -----------{}",
             green(),
             title,
             reset(),
    );

    if collection.len() == 0 {
        println!("No projects found. Try creating a new project.");
        println!("{}commando projects --create <NAME>\n{}", red(), reset());
    }

    for project in collection {
        println!("- {}", project.name);
    }
}

fn initiate_matches() -> ArgMatches<'static> {
    let m = CliApp::new("Commando")
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

    // Projects
        .subcommand(SubCommand::with_name("projects")
                    .about("manage projects within commando")
                    .subcommand(SubCommand::with_name("services")
                                .about("Manage services")

                                .arg(Arg::with_name("enable")
                                     .short("e")
                                     .long("enable")
                                     .takes_value(true)
                                     .value_name("SERVICE_NAME")
                                     .help("Enable specific service"))

                                .arg(Arg::with_name("remove")
                                     .short("r")
                                     .long("remove")
                                     .takes_value(true)
                                     .value_name("SERVICE_NAME")
                                     .help("Remove specific service"))

                                .arg(Arg::with_name("start")
                                     .short("s")
                                     .long("start")
                                     .takes_value(true)
                                     .value_name("SERVICE_NAME")
                                     .help("Start specific service"))

                                .arg(Arg::with_name("list")
                                     .short("l")
                                     .long("list")
                                     .help("List all services")))

                    .arg(Arg::with_name("list")
                         .short("l")
                         .long("list")
                         .help("List all projects"))

                    .arg(Arg::with_name("create")
                         .short("c")
                         .long("create")
                         .help("Create a new project")
                         .value_name("PROJECT_NAME")
                         .takes_value(true))

                    .arg(Arg::with_name("promote")
                        .short("p")
                        .long("promote")
                        .value_name("PROJECT_NAME")
                        .help("Designate a default project")
                         .takes_value(true))

                    .arg(Arg::with_name("info")
                        .short("i")
                        .long("info")
                         .value_name("PROJECT_NAME")
                        .help("Information about a project")
                         .takes_value(true))

                    .arg(Arg::with_name("setup")
                         .short("s")
                         .long("setup")
                         .value_name("PROJECT_NAME")
                         .help("Trigger project setup routines")
                         .takes_value(true))

                    .arg(Arg::with_name("purge")
                        .short("x")
                        .long("purge")
                         .value_name("PROJECT_NAME")
                        .help("Purge a project's data and file directory")
                        .takes_value(true)))

    // Deploy
        .subcommand(SubCommand::with_name("deploy")
                    .about("deployment")
                    .arg(Arg::with_name("debug")
                         .short("d")
                         .help("print debug information verbosely"))

                    .arg(Arg::with_name("service")
                         .short("s")
                         .long("service")
                         .value_name("SERVICE_NAME")
                         .help("Initiate deploy protocol for a given service")
                         .takes_value(true))

                    .arg(Arg::with_name("action")
                         .short("a")
                         .long("action")
                         .value_name("ACTION")
                         .help("Dictate deployment action for a given service")
                         .takes_value(true)))

    // Monitor
        .subcommand(SubCommand::with_name("monitor")
                    .about("live status monitor"))

    // Logs
        .subcommand(SubCommand::with_name("logs")
                    .about("status")
                    .arg(Arg::with_name("debug")
                         .short("d")
                         .help("print debug information verbosely")))

    // Utiltiies
        .subcommand(SubCommand::with_name("config")
                    .about("Configuration tools")
                    .subcommand(SubCommand::with_name("compile")
                                .about("Configuration tools"))

                    .arg(Arg::with_name("target")
                         .short("t")
                         .long("target")
                         .help("Target file to be processed")
                         .takes_value(true))

                    .arg(Arg::with_name("output")
                         .short("o")
                         .long("output")
                         .help("Output file")
                         .takes_value(true)))

        .get_matches();

    return m;
}
