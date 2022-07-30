// Logger
use owo_colors::OwoColorize;
use paris::{error, info, success};

// Search the Pat&h
use search_path::SearchPath;
use std::path::PathBuf;

pub fn get_path(args: Vec<&str>) {
  info!("Executables to get: {}", args.join(", "));
  // Get the PATH to search
  let path: &str;

  if cfg!(windows) {
    path = "CMD_PATH";
  } else if cfg!(unix) {
    path = "PATH";
  } else {
    error!("Not supported OS only support windows and unix!");
    std::process::exit(1);
  }

  // Search
  for args in args.iter() {
    let path = SearchPath::new(path).expect("WTF Is necessary a $PATH dir!!");
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
