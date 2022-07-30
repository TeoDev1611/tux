use clap::{Arg, ArgAction, Command};
pub mod tux;

fn main() {
    let app = Command::new("tux")
        .version("0.1.0")
        .author("TeoDev1611")
        .about("Tux the way to use the linux | unix commands into multiple platforms")
        .arg_required_else_help(true)
        .subcommand_required(true)
        // The Touch command setup
        .subcommand(
            Command::new("touch")
                .short_flag('T')
                .long_flag("touch")
                .about("The basic touch command create the file in the route provided")
                .arg(Arg::new("path").help(
                    "The path to create the file\nExample: tux touch ./a_very_important_file.txt").action(ArgAction::Set).multiple_values(true)),
        )
        .get_matches();

    match app.subcommand() {
        Some(("touch", touch_matches)) => {
            // Get the arguments in a vector of strings
            let files: Vec<&str> = touch_matches
                .get_many::<String>("path")
                .expect("The expected parameter is a string")
                .map(|s| s.as_str())
                .collect();

            // Tux function for the touch command
            tux::touch::write_file(files);
        }
        _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
    }
}
