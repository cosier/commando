
use project::{ProjectData};
use utils::{green, red, reset};

// Render a list of Projects
pub fn display_as_list(title: &str, mut collection: Vec<Box<ProjectData>>) {
    println!("\n{}----------- {}{}",
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
