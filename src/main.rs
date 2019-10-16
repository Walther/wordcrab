use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    /// Files to process
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();
    if opt.debug {
        println!("{:#?}", opt);
    }

    for file in opt.files {
        let file = File::open(file)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;

        let chars = contents.chars().count();
        let lines = contents.lines().count();
        let words = contents.split_whitespace().count();

        println!("{} lines {} words {} chars", lines, words, chars);
    }

    Ok(())
}
