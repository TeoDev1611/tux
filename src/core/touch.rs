use clap::ArgMatches;
use std::fs::File;
use std::io::{BufWriter, Write};
use colored::*;

pub fn write_file(args: &ArgMatches) {
    let name = args.value_of("touch").unwrap();
    let data = "";
    let f = File::create(name).expect("Unable to write a file");
    let mut f = BufWriter::new(f);
    f.write_all(data.as_bytes())
        .expect("Unable to write the data");
    println!("{}", "Successfully created the file :p".green());
}
