use clap::ArgMatches;
pub const NO_PROJECT_SELECTED: &str = "cli_no_project_selected";


pub mod services;
pub mod projects;
pub mod tree;

use project::active_project;
use cli::tree::build_tree as tree;
use cli::services::parse_services;
use cli::projects::parse_projects;
use utils::print_help;
use environment::initialize_environment;

pub struct Processor {}

impl Processor {
    pub fn new() -> Processor {
        Processor {}
    }

    /// Parses the arguments given via cli
    pub fn parse(&self) {
        let root: ArgMatches = tree().get_matches();
        initialize_environment(&root);

        let active = &active_project().unwrap_or(NO_PROJECT_SELECTED.to_string())[..];
        let project_id: &str = root.value_of("project").unwrap_or(active);

        // Projects management
        parse_projects(project_id, &root);

        // Services management
        parse_services(project_id, &root);

        print_help();
    }
}
