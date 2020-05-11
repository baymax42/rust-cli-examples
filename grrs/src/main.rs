use std::fs::File;
use std::io::{BufReader, prelude::*};
use std::path::PathBuf;

use exitfailure::ExitFailure;
use failure::ResultExt;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt()]
struct CliOptions {
    /// The pattern to look for inside the file
    pattern: String,

    #[structopt(parse(from_os_str))]
    /// The path to the file in which you'd like to search
    path: PathBuf,
}

fn main() -> Result<(), ExitFailure> {
    let options: CliOptions = CliOptions::from_args();
    let path = &options.path;

    let file = File::open(path)
        .with_context(|_| format!("could not read the file {:?}", path))?;
    let reader = BufReader::new(file);

    for (num, line) in reader.lines().enumerate() {
        let content = line.with_context(|_| format!("could not read line {}", num))?;

        if content.contains(&options.pattern) {
            println!("{}", content);
        }
    }

    Ok(())
}
