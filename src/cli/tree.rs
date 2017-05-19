
use clap::{App, Arg, SubCommand, ArgMatches};


pub fn build_tree<'a>() -> App<'a,'a> {
    let app = App::new("commando")
        .version("0.1-alpha")
        .author("Bailey Cosier. <bailey@crowdist.com>")
        .about("Commando - Swiss army knife management cli");

    let tree = Tree { app: app };

    tree
        .projects()
        .services()
        .deploy()
        .config()
        .monitor()
        .logs()
        .gui()
        .app
}

pub struct Tree<'a> {
    app: App<'a, 'a>
}

impl<'c> Tree<'c> {

    fn gui(self) -> Self {
        Tree {
            app: self.app.subcommand(SubCommand::with_name("gui")
                                     .about("open a dynamic ui based on ncurses")
                                     .arg(Arg::with_name("debug")
                                          .short("d")
                                          .help("print debug information verbosely")))
        }
    }

    fn projects(self) -> Self {
        Tree {
            app: self.app.arg(Arg::with_name("project")
                              .short("p")
                              .long("project")
                              .value_name("PROJECT_NAME")
                              .help("Specify project explicitly")
                              .takes_value(true))
                .subcommand(SubCommand::with_name("projects")
                            .about("manage projects within commando")

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
        }
    }

    fn services(self) -> Self {
        Tree {
            app: self.app.subcommand(SubCommand::with_name("services")
                        .about("Manage services")

                        .arg(Arg::with_name("enable")
                             .long("enable")
                             .takes_value(true)
                             .value_name("SERVICE_NAME")
                             .help("Enable specific service"))

                        .arg(Arg::with_name("remove")
                             .long("remove")
                             .takes_value(true)
                             .value_name("SERVICE_NAME")
                             .help("Remove specific service"))

                        .arg(Arg::with_name("start")
                             .long("start")
                             .takes_value(true)
                             .value_name("SERVICE_NAME")
                             .help("Start specific service"))

                        .arg(Arg::with_name("logs")
                             .long("logs")
                             .takes_value(true)
                             .value_name("SERVICE_NAME")
                             .help("Logs for a specific service"))

                        .arg(Arg::with_name("attach")
                             .long("attach")
                             .takes_value(true)
                             .value_name("SERVICE_NAME")
                             .help("Attach to a specific service"))

                        .arg(Arg::with_name("env")
                             .long("env")
                             .takes_value(true)
                             .value_name("SERVICE_NAME")
                             .help("View Environment settings for a service"))

                        .arg(Arg::with_name("list")
                             .long("list")
                             .help("List all services")))
        }
    }

    fn deploy(self) -> Self {
        Tree { app: self.app.subcommand(SubCommand::with_name("deploy")
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
        }
    }

    fn monitor(self) -> Self {
        Tree { app: self.app.subcommand(SubCommand::with_name("monitor")
                                        .about("live status monitor")) }
    }

    fn logs(self) -> Self {
        Tree { app: self.app.subcommand(SubCommand::with_name("logs")
                        .about("status")
                        .arg(Arg::with_name("debug")
                             .short("d")
                             .help("print debug information verbosely"))) }
    }

    fn config(self) -> Self {
        Tree { app: self.app.subcommand(SubCommand::with_name("config")
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
                                       .takes_value(true))) }
    }
}
