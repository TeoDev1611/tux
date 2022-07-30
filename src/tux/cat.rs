// The external modules
use owo_colors::OwoColorize;
use paris::{error, info, success};

// The std modules for the files
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn read_file(args: Vec<&str>) {
  info!("Files to read: {}\n", args.join(", "));
  for args in args.iter() {
    let path = Path::new(args);
    // Try read the files
    let mut file = match File::open(&path) {
      Err(why) => {
        error!(
          "Couldn't open the file {}\nBecause: {}",
          path.display(),
          why
        );
        std::process::exit(1);
      }
      Ok(file) => file,
    };

    let mut content = String::new();
    match file.read_to_string(&mut content) {
      Err(why) => {
        error!(
          "Couldn't open the file {}\nBecause: {}",
          path.display(),
          why
        );
        std::process::exit(1);
      }
      Ok(_) => {
        success!("Successfully read the file: {}\n", path.display());
        println!("{}: \n{}", "CONTENT".bright_green().underline(), content);
        println!(
          "{}",
          "--------------------------------------------".bright_purple()
        )
      }
    }
  }
}
