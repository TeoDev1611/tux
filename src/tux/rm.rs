use paris::{error, info, success};

// The std modules for the files
use std::fs::{metadata, remove_dir_all, remove_file};
use std::path::Path;

pub fn delete_file(args: Vec<&str>) {
  info!("Files to remove: {}\n", args.join(", "));
  for args in args.iter() {
    // Route
    let path = Path::new(args);
    // Get info
    let md = metadata(path).unwrap();

    // Delete
    if md.is_dir() {
      info!("Directory detected for: {}", path.display());
      remove_dir_all(path).expect("Can't delete the directory");
      success!("Deleted successfully the directory {}", path.display());
    } else if md.is_file() {
      info!("File detected for: {}", path.display());
      remove_file(path).expect("Can't delete the file");
      success!("Deleted successfully the file {}", path.display());
    } else {
      error!("Can't delete this isn't a file or an directory!");
      std::process::exit(1);
    }
  }
}
