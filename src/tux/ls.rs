use paris::{info, success};

// List walk dir
use walkdir::{DirEntry, WalkDir};

fn is_hidden(entry: &DirEntry) -> bool {
  entry
    .file_name()
    .to_str()
    .map(|s| s.starts_with(".") || s.starts_with("target"))
    .unwrap_or(false)
}

pub fn list_directories() {
  info!(
    "Showing the files in the path: {}",
    std::env::current_dir().unwrap().display()
  );
  let walker = WalkDir::new(std::env::current_dir().unwrap()).into_iter();
  for entry in walker.filter_entry(|e| !is_hidden(e)) {
    let entry = entry.unwrap();
    println!("{}", entry.path().display());
  }
  success!("Check this is all files in the current directory!!")
}
