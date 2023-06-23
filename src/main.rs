#![deny(clippy::all)]
use clap::Parser;
use rayon::prelude::*;
use serde_json::json;

use std::error::Error;
use std::path::PathBuf;

use wordcrab::{analyse_file, analyse_files, AnalysisOptions};

#[derive(Parser, Debug)]
#[clap(
    name = "wordcrab",
    about = "A command-line tool for counting lines, words and characters in documents.

It is a modern, cross-platform replacement for wc(1).

When analysis options (any combination of -l, -w, -c) are specified, wordcrab only analyses and reports the information requested. The order of output is always line, word, chars, filename. By default, -lwc is assumed.

Multiple output formats are supported in addition to the standard text output.
"
)]
struct Opt {
    /// Activate debug mode
    #[clap(short, long)]
    debug: bool,

    /// Count the number of lines.
    /// Based on \n and \r\n
    #[clap(short, long)]
    lines: bool,

    /// Count the number of words.
    /// Based on the Unicode Derived Core Property White_Space
    #[clap(short, long)]
    words: bool,

    /// Count the number of chars.
    /// Based on the Unicode Scalar Value
    #[clap(short, long)]
    chars: bool,

    /// Select the output format
    #[clap(short, long, value_parser = ["text", "json", "yaml", "toml"], default_value = "text")]
    output: String,

    /// Files to process
    #[clap(name = "FILE", value_parser)]
    files: Vec<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut opt = Opt::parse();
    if opt.debug {
        println!("raw opts:");
        println!("{:#?}", opt);
    }

    // If none of the analysis options are specified, count and report all of them.
    // Otherwise, only the specified ones.
    if !opt.lines && !opt.words && !opt.chars {
        opt.lines = true;
        opt.words = true;
        opt.chars = true;
    };
    let analysis_options = AnalysisOptions {
        lines: opt.lines,
        words: opt.words,
        chars: opt.chars,
    };
    if opt.debug {
        println!("parsed analysis options:");
        println!("{:#?}", analysis_options);
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
            println!("{}", analyse_file(filename, analysis_options));
        }),
        "json" => {
            let results = analyse_files(&filenames, analysis_options);
            println!("{}", json!(results))
        }
        "toml" => {
            let results = analyse_files(&filenames, analysis_options);
            println!("{}", toml::to_string(&results)?)
        }
        "yaml" => {
            let results = analyse_files(&filenames, analysis_options);
            match serde_yaml::to_string(&results) {
                Ok(yaml) => println!("{}", yaml),
                Err(e) => panic!("{}", e),
            }
        }
        _ => unreachable!(), // clap has explicit list of possible_values and a default_value
    }
    Ok(())
}
