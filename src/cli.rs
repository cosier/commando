use clap;
use clap::{Arg, SubCommand, ArgMatches};
use db::Database as DB;

use std;

use std::path::PathBuf;
use slug;

use project::{
    create_project,
    promote_project,
    info_project,
    purge_project,
    setup_project,
    active_project,

    project_service_start,
    project_service_stop,
    project_service_restart,
    project_service_enable,
    project_service_disable,
    project_service_logs,
    project_service_env,
    project_service_list
};

use views::{display_as_list};
const NO_PROJECT_SELECTED: &str = "cli_no_project_selected";

fn exit(msg: &str) {
    error!("{}", msg);
    std::process::exit(0);
}

fn make_absolute(path: &str) -> String {
    if path.find('/').unwrap() == 0 {
        debug!("detected absolute path: {}", path);
        return path.to_string();
    } else {
        let current = std::env::current_dir().unwrap();
        let str : String = format!("{}/{}", current.to_str().unwrap(), path);
        return str;
    }

}

pub struct Processor {}
impl Processor {
    pub fn new() -> Processor {
        return Processor {}
    }

    /// Parses the arguments given via cli
    pub fn parse(&self) {
        let root: ArgMatches = self.initiate_matches();
        let active = &active_project().unwrap_or(NO_PROJECT_SELECTED.to_string())[..];
        let project_id : &str = root.value_of("project").unwrap_or(active);

        // List Projects
        if let Some(matches) = root.subcommand_matches("projects") {

            // Create a new project
            match matches.occurrences_of("create") {
                1  => {
                    let project_name = &slug::slugify(
                        matches.value_of("create").unwrap())[..];

                    let mut path : PathBuf = std::env::current_dir().unwrap();

                    // Check for explicit root specified during this command
                    match matches.value_of("barge-root") {
                        Some(p) => {
                            path = PathBuf::from(make_absolute(p));
                            debug!("barge-root detected: {}", path.to_str().unwrap());
                        },
                        _ => {
                            path = PathBuf::from(format!(
                                "{}/{}",
                                path.to_str().unwrap(),
                                project_name))
                        }
                    }

                    // Send to project interface
                    create_project(project_name, path);
                },
                _  => ()
            }

            // Promote a project
            match matches.occurrences_of("promote") {
                1  => promote_project(matches.value_of("promote").unwrap()),
                _  => ()
            }

            // Describe a project
            match matches.occurrences_of("info") {
                1  => info_project(matches.value_of("info").unwrap()),
                _  => ()
            }

            // Purge a project
            match matches.occurrences_of("purge") {
                1  => purge_project(matches.value_of("purge").unwrap()),
                _  => ()
            }

            // Process setup of a project
            match matches.occurrences_of("setup") {
                1  => setup_project(matches.value_of("setup").unwrap()),
                _  => ()
            }

            // Service management per project
            if let Some(sm) = matches.subcommand_matches("services") {

                // Check for no active project detected, and bail if necessary.
                if project_id == NO_PROJECT_SELECTED {
                    let opt: String = active_project().unwrap_or(NO_PROJECT_SELECTED.to_string());
                    if opt == NO_PROJECT_SELECTED {
                        exit("Need to specify project or designate a default project");
                    }
                }

                match sm.occurrences_of("start") {
                    1 => {
                        let service_name = sm.value_of("start").unwrap();
                        project_service_start(project_id, service_name)
                    },
                    _ => ()
                }

                match sm.occurrences_of("stop") {
                    1 => {
                        let service_name = sm.value_of("stop").unwrap();
                        project_service_stop(project_id, service_name)
                    },
                    _ => ()
                }

                match sm.occurrences_of("restart") {
                    1 => {
                        let service_name = sm.value_of("stop").unwrap();
                        project_service_restart(project_id, service_name)
                    },
                    _ => ()
                }

                match sm.occurrences_of("enable") {
                    1 => {
                        let service_name = sm.value_of("enable").unwrap();
                        project_service_enable(project_id, service_name)
                    },
                    _ => ()
                }

                match sm.occurrences_of("disable") {
                    1 => {
                        let service_name = sm.value_of("disable").unwrap();
                        project_service_disable(project_id, service_name)
                    },
                    _ => ()
                }

                match sm.occurrences_of("logs") {
                    1 => {
                        let service_name = sm.value_of("logs").unwrap();
                        project_service_logs(project_id, service_name)
                    },
                    _ => ()
                }

                match sm.occurrences_of("env") {
                    1 => {
                        let service_name = sm.value_of("env").unwrap();
                         project_service_env(project_id, service_name)
                    },
                    _ => ()
                }

                match sm.occurrences_of("list") {
                    1 => {
                        project_service_list(project_id)
                    },
                    _ => ()
                }

            }

            display_as_list("Projects", DB::list_projects());
        }
    }

    fn initiate_matches(&self) -> ArgMatches {
        let composed = self.build_tree();
        composed.get_matches()
    }

    fn build_tree(&self) -> clap::App {
        clap::App::new("Commando")
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

                                    .arg(Arg::with_name("logs")
                                         .short("l")
                                         .long("logs")
                                         .takes_value(true)
                                         .value_name("SERVICE_NAME")
                                         .help("Logs for a specific service"))

                                    .arg(Arg::with_name("attach")
                                         .short("a")
                                         .long("attach")
                                         .takes_value(true)
                                         .value_name("SERVICE_NAME")
                                         .help("Attach to a specific service"))

                                    .arg(Arg::with_name("env")
                                         .short("e")
                                         .long("env")
                                         .takes_value(true)
                                         .value_name("SERVICE_NAME")
                                         .help("View Environment settings for a service"))

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

                        .arg(Arg::with_name("barge-root")
                             .long("barge-root")
                             .help("Set barge root for installation")
                             .value_name("BARGE_ROOT")
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
    }
}
