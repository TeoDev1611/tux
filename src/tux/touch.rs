// Logger
use paris::{error, info, success};
// File manager operations
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

pub fn write_file(args: Vec<&str>) {
    info!("Files to create: {}", args.join(", "));
    for args in args.iter() {
        let path = Path::new(args);
        // Try write the file
        let mut file = match File::create(&path) {
            Err(why) => {
                error!("Couldn't create the file {}\nBecause: {}", path.display(), why);
                std::process::exit(1);
            }
            Ok(file) => file,
        };

        match file.write_all("".as_bytes()) {
            Err(why) => {
                error!("Couldn't create the file {}\nBecause: {}", path.display(), why);
                std::process::exit(1);
            }
            Ok(_) => success!("Successfully wroted to: {}", path.display()),
        }
    }
}
