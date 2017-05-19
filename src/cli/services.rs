use std;
use std::path::PathBuf;
use clap::{ArgMatches};

use project::{
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

use utils::{make_absolute, exit};

use cli::tree::build_tree as tree;
use cli::{NO_PROJECT_SELECTED};

pub fn parse_services(project_id: &str, root: &ArgMatches) {
    if let Some(matches) = root.subcommand_matches("services") {

        tree().print_help();
        // Check for no active project detected, and bail if necessary.
        if project_id == NO_PROJECT_SELECTED {
            let opt: String = active_project().unwrap_or(NO_PROJECT_SELECTED.to_string());
            if opt == NO_PROJECT_SELECTED {
                exit("Need to specify project or designate a default project");
            }
        }

        match matches.occurrences_of("start") {
            1 => {
                let service_name = matches.value_of("start").unwrap();
                project_service_start(project_id, service_name)
            },
            _ => ()
        }

        match matches.occurrences_of("stop") {
            1 => {
                let service_name = matches.value_of("stop").unwrap();
                project_service_stop(project_id, service_name)
            },
            _ => ()
        }

        match matches.occurrences_of("restart") {
            1 => {
                let service_name = matches.value_of("stop").unwrap();
                project_service_restart(project_id, service_name)
            },
            _ => ()
        }

        match matches.occurrences_of("enable") {
            1 => {
                let service_name = matches.value_of("enable").unwrap();
                project_service_enable(project_id, service_name)
            },
            _ => ()
        }

        match matches.occurrences_of("disable") {
            1 => {
                let service_name = matches.value_of("disable").unwrap();
                project_service_disable(project_id, service_name)
            },
            _ => ()
        }

        match matches.occurrences_of("logs") {
            1 => {
                let service_name = matches.value_of("logs").unwrap();
                project_service_logs(project_id, service_name)
            },
            _ => ()
        }

        match matches.occurrences_of("env") {
            1 => {
                let service_name = matches.value_of("env").unwrap();
                project_service_env(project_id, service_name)
            },
            _ => ()
        }

        match matches.occurrences_of("list") {
            1 => {
                project_service_list(project_id)
            },
            _ => ()
        }

    }
}
