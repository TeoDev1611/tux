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
        .about("The basic touch command create the file in the route provided")
        .arg(
          Arg::new("path")
            .help("The path to create the file\nExample: tux touch ./a_very_important_file.txt")
            .action(ArgAction::Set)
            .multiple_values(true),
        ),
    )
    .subcommand(
      Command::new("cat")
        .about("The basic cat command improved by read many files")
        .arg(
          Arg::new("path")
            .help("The path to read the files\nExample: tux cat ./another_file_to_show.txt")
            .action(ArgAction::Set)
            .multiple_values(true),
        ),
    )
    .subcommand(
      Command::new("rm")
        .about("The basic rm command improved by automatic detect if is file or dir")
        .arg(
          Arg::new("path")
            .help("The path to read the files\nExample: tux rm ./another_file_to_remove.txt")
            .action(ArgAction::Set)
            .multiple_values(true),
        ),
    )
    .subcommand(
      Command::new("which")
        .about("The basic whic command improved by get many executables")
        .arg(
          Arg::new("path")
            .help("The path to read the files\nExample: tux which cargo")
            .action(ArgAction::Set)
            .multiple_values(true),
        ),
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
    Some(("cat", cat_matches)) => {
      let files: Vec<&str> = cat_matches
        .get_many::<String>("path")
        .expect("The expected parameter is a string")
        .map(|s| s.as_str())
        .collect();

      // Tux function for the cat command
      tux::cat::read_file(files);
    }
    Some(("rm", rm_matches)) => {
      let files: Vec<&str> = rm_matches
        .get_many::<String>("path")
        .expect("The expected parameter is a string")
        .map(|s| s.as_str())
        .collect();

      // Tux function for the rm command
      tux::rm::delete_file(files)
    }
    Some(("which", which_matches)) => {
      let files: Vec<&str> = which_matches
        .get_many::<String>("path")
        .expect("The expected parameter is a string")
        .map(|s| s.as_str())
        .collect();

      // Tux function for the rm command
      tux::which::get_path(files)
    }
    _ => unreachable!(), // If all subcommands are defined above, anything else is unreachable
  }
}
