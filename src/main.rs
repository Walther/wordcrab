#![deny(clippy::all)]
use clap::AppSettings;
use rayon::prelude::*;
use serde_json::json;
use serde_yaml;
use std::path::PathBuf;
use structopt::StructOpt;

pub mod lib;
use lib::{analyse_file, analyse_files};

#[derive(StructOpt, Debug)]
#[structopt(
    name = "wordcrab",
    about = "A command-line tool for counting lines, words and characters in documents.",
    global_settings = &[AppSettings::ColoredHelp]
)]
struct Opt {
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    /// Select the output format
    #[structopt(short, long, possible_values = &["text", "json", "yaml"], default_value = "text")]
    output: String,

    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    if opt.debug {
        println!("{:#?}", opt);
    }
    let files = opt.files;
    let filenames: Vec<&str> = files
        .par_iter()
        .map(|path| path.to_str().unwrap()) // FIXME: don't panic?
        .collect();

    // If output format is text, we can stream-print as we go.
    // If output format is specified to any other format, we'll collect values first
    // in order to output a correct file
    match opt.output.as_str() {
        "text" => filenames.par_iter().for_each(|filename| {
            println!("{}", analyse_file(&filename.to_string()));
        }),
        "json" => {
            let results = analyse_files(&filenames);
            println!("{}", json!(results))
        }
        "yaml" => {
            let results = analyse_files(&filenames);
            match serde_yaml::to_string(&results) {
                Ok(yaml) => println!("{}", yaml),
                Err(e) => panic!("{}", e),
            }
        }
        _ => unreachable!(), // structopt has explicit list of possible_values and a default_value
    }
    Ok(())
}
