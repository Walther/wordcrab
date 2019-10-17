#[macro_use]
extern crate clap;
use serde_json::json;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Clap, Debug)]
#[clap(
    name = "wordcrab",
    about = "A command-line tool for counting lines, words and characters in documents.",
    // global_settings = &[AppSettings::ColoredHelp]
)]
struct Opt {
    /// Activate debug mode
    #[clap(short = "d", long = "debug")]
    debug: bool,

    /// Select the output format
    #[clap(short = "o", long = "output", /* possible_values = &["text", "json"], */ default_value = "text")]
    output: String,

    /// Files to process
    #[clap(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() -> std::io::Result<()> {
    let opt = Opt::parse();
    if opt.debug {
        println!("{:#?}", opt);
    }

    for path in opt.files {
        let filename = path.to_str().unwrap();
        let file = File::open(filename)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;

        let chars = contents.chars().count();
        let lines = contents.lines().count();
        let words = contents.split_whitespace().count();

        match opt.output.as_str() {
            "json" => {
                let json = json!({
                    "lines": lines,
                    "words": words,
                    "chars": chars,
                    "filename": filename
                });
                println!("{}", json.to_string());
            }
            _ => {
                println!("{} {} {} {}", lines, words, chars, filename);
            }
        }
    }

    Ok(())
}
