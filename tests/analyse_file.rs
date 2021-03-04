extern crate wordcrab;
use wordcrab::NamedOutput::{Error, Success};
use wordcrab::*;

mod analysis_options;
use analysis_options::*;

#[test]
fn long_file() {
    let named_output = analyse_file("tests/content/seitseman_veljesta.txt", ANALYSIS_OPTIONS_LWC);
    match named_output {
        Success { filename, stats } => {
            println!("{}", filename);
            assert_eq!(stats.lines, Some(13681));
            assert_eq!(stats.words, Some(84192));
            assert_eq!(stats.chars, Some(661759));
        }
        Error { filename: _, error } => {
            panic!(error);
        }
    }
}
