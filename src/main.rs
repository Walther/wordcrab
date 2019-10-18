use clap::AppSettings;
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

    for path in opt.files {
        let filename = path.to_str().unwrap();

        match analyse_file(&filename) {
            Ok(file_stats) => match opt.output.as_str() {
                "json" => {
                    let json = json!({
                        "lines": file_stats.lines,
                        "words": file_stats.words,
                        "chars": file_stats.chars,
                        "filename": file_stats.filename
                    });
                    println!("{}", json.to_string());
                }
                _ => {
                    println!(
                        "{} {} {} {}",
                        file_stats.lines, file_stats.words, file_stats.chars, file_stats.filename
                    );
                }
            },
            Err(error) => match opt.output.as_str() {
                "json" => {
                    let json = json!({
                        "filename": filename,
                        "error": error.to_string()
                    });
                    println!("{}", json.to_string());
                }
                _ => {
                    println!("{} {}", error.to_string(), filename);
                }
            },
        }
    }

    Ok(())
}
