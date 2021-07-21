use clap::{App, Arg, SubCommand};
pub mod core;

fn main() {
    let app = App::new("Tux")
        .version("0.1.0")
        .author("TeoDev1611")
        .about("Tux the way to use the linux | unix commands into multiple platforms")
        .subcommand(
            SubCommand::with_name("touch")
                .about("Write a file with a name touch :p")
                .version("0.1.0")
                .author("TeoDev1611")
                .help("Write a file passing the name tux touch foo.txt")
                .arg(Arg::with_name("touch").takes_value(true).required(true)),
        )
        .subcommand(
            SubCommand::with_name("ls")
                .about(
                    "Show all files and directories in the current directory like ls in linux :p",
                )
                .version("0.1.0")
                .author("TeoDev1611")
                .help("Write tux ls for show all files and write tux ls -la"),
        );

    let matches = app.clone().get_matches();

    match matches.subcommand() {
        ("touch", Some(args)) => core::touch::write_file(args),
        _ => {
            app.clone()
                .print_help()
                .expect("Cannot print the help message");
            println!();
        }
    }
}
