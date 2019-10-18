use clap::AppSettings;
use rayon::prelude::*;
use serde_json::json;
use std::path::PathBuf;
use structopt::StructOpt;

pub mod wordcrab;
use wordcrab::analyse_file;

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
    #[structopt(short, long, possible_values = &["text", "json"], default_value = "text")]
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

    opt.files.par_iter().for_each(|path| {
        let filename = path.to_str().unwrap();

        match analyse_file(&filename) {
            Ok(file_stats) => match opt.output.as_str() {
                "json" => {
                    let json = json!(file_stats);
                    println!("{}", json);
                }
                _ => {
                    println!("{}", file_stats);
                }
            },
            Err(error) => match opt.output.as_str() {
                "json" => {
                    let json = json!(error);
                    println!("{}", json);
                }
                _ => {
                    println!("{}", error);
                }
            },
        }
    });
    Ok(())
}
