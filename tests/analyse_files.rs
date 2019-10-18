extern crate wordcrab;
use wordcrab::NamedOutput::{Error, Success};
use wordcrab::*;

#[test]
fn multiple_empty_files() {
  let files = [
    "tests/content/empty_a.txt",
    "tests/content/empty_b.txt",
    "tests/content/empty_c.txt",
  ];
  let results = analyse_files(&files);
  for named_output in results {
    match named_output {
      Success { filename, stats } => {
        println!("{}", filename);
        assert_eq!(stats.lines, 0);
        assert_eq!(stats.words, 0);
        assert_eq!(stats.chars, 0);
      }
      Error { filename: _, error } => {
        panic!(error);
      }
    }
  }
}
