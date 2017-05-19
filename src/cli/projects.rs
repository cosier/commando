
use std;
use std::path::PathBuf;
use clap::{ArgMatches, App};

use project::{
    create_project,
    promote_project,
    info_project,
    purge_project,
    setup_project,
    active_project,
};

use utils::{make_absolute, if_occurred, print_help};
use cli::tree::build_tree as tree;
use slug;

pub fn parse_projects(project_id: &str, root: &ArgMatches) {


    if let Some(matches) = root.subcommand_matches("projects") {

        // Create a new project
        if_occurred("create", matches, || {
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
            create_project(project_name, path)
        });

        // Promote a project
        if_occurred("promote", matches, || {
            promote_project(matches.value_of("promote").unwrap())
        });

        // Describe a project
        if_occurred("info", matches, || {
            info_project(matches.value_of("info").unwrap())
        });

        // Purge a project
        if_occurred("purge", matches, || {
            purge_project(matches.value_of("purge").unwrap())
        });

        // Process setup of a project
        if_occurred("setup", matches, || {
            setup_project(matches.value_of("setup").unwrap())
        });

        print_help();
    }
}
