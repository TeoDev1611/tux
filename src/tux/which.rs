// Logger
use owo_colors::OwoColorize;
use paris::{error, info, success};

// Search the Pat&h
use search_path::SearchPath;
use std::path::PathBuf;

pub fn get_path(args: Vec<&str>) {
  info!("Executables to get: {}", args.join(", "));
  for args in args.iter() {
    let path = SearchPath::new("PATH").expect("WTF Is necessary a $PATH dir!!");
    let req = path.find_file(&PathBuf::from(args));
    match req {
      Some(p) => {
        success!("Successfully found the {} in the PATH", args);
        println!("{}", p.display().underline());
      }
      None => {
        error!(
          "Not found the executable of {} in the PATH!!",
          args.to_uppercase().underline()
        );
        std::process::exit(1)
      }
    }
  }
}
