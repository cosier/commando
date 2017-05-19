
use std;
use std::path::PathBuf;
use clap::{ArgMatches};

use project::{
    create_project,
    promote_project,
    info_project,
    purge_project,
    setup_project,
    active_project,
};

use utils::{make_absolute, exit};
use cli::tree::build_tree as tree;
use slug;

pub fn parse_projects(project_id: &str, root: &ArgMatches) {
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

        tree().print_help();
    }
}
