// use std;
// use std::path::PathBuf;
use clap::{ArgMatches};

use project::{
    active_project,
};

use service::{
    service_start,
    service_stop,
    service_restart,
    service_enable,
    service_disable,
    service_logs,
    service_env,
    service_list
};

use utils::{exit, if_occurred, print_help};
// use cli::tree::build_tree as tree;
use cli::{NO_PROJECT_SELECTED};

pub fn parse_services(project_id: &str, root: &ArgMatches) {
    if let Some(matches) = root.subcommand_matches("services") {
        // Check for no active project detected, and bail if necessary.
        if project_id == NO_PROJECT_SELECTED {
            let opt: String = active_project().unwrap_or(NO_PROJECT_SELECTED.to_string());
            if opt == NO_PROJECT_SELECTED {
                exit();
            }
        }

        if_occurred("start", matches, || {
            let service_name = matches.value_of("start").unwrap();
            service_start(project_id, service_name)
        });

        if_occurred("stop", matches, || {
            let service_name = matches.value_of("stop").unwrap();
            service_stop(project_id, service_name)
        });

        if_occurred("restart", matches, || {
            let service_name = matches.value_of("restart").unwrap();
            service_restart(project_id, service_name)
        });

        if_occurred("enable", matches, || {
            let service_name = matches.value_of("enable").unwrap();
            service_enable(project_id, service_name)
        });

        if_occurred("disable", matches, || {
            let service_name = matches.value_of("disable").unwrap();
            service_disable(project_id, service_name)
        });

        if_occurred("logs", matches, || {
            let service_name = matches.value_of("logs").unwrap();
            service_logs(project_id, service_name)
        });

        if_occurred("env", matches, || {
            let service_name = matches.value_of("env").unwrap();
            service_env(project_id, service_name)
        });

        if_occurred("list", matches, || {
            service_list(project_id)
        });

        print_help();
    }
}
