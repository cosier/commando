
use clap::{Arg, App as CliApp, SubCommand, ArgMatches};

pub fn parse() {
    let matches: ArgMatches = matches();
    let project_id = matches.value_of("project").unwrap_or("none");
    info!("project_id: {}", project_id);

    
}

fn matches() -> ArgMatches<'static> {
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

    return m;
}
