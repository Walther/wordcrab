extern crate wordcrab;
use wordcrab::NamedOutput::{Error, Success};
use wordcrab::*;

#[test]
fn long_file() {
  // FIXME: these values might be wrong due to two-character
  // representations of the symbols ä and ö
  let named_output = analyse_file("tests/content/seitseman_veljesta.txt");
  match named_output {
    Success { filename, stats } => {
      println!("{}", filename);
      assert_eq!(stats.lines, 13681);
      assert_eq!(stats.words, 84192);
      assert_eq!(stats.chars, 661759);
    }
    Error { filename: _, error } => {
      panic!(error);
    }
  }
}
