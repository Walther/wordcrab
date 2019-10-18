use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

/// Structure representing the results of a file analysis.
pub struct FileStats {
  /// String representation of the filename given as the input, including possible relative path
  pub filename: String,
  /// Number of lines in the file
  pub lines: usize,
  /// Number of words in the file. TODO: this needs to be strictly defined!
  pub words: usize,
  /// Number of characters in the file. TODO: this needs to be strictly defined!
  pub chars: usize,
}

/// Runs a file analysis on the given filename path. Returns a FileStats structure representing the results, or an IO Error.
pub fn analyse_file(filename: &str) -> Result<FileStats, std::io::Error> {
  let file = File::open(filename)?;
  let mut buf_reader = BufReader::new(file);
  let mut contents = String::new();
  buf_reader.read_to_string(&mut contents)?;

  let chars = contents.chars().count();
  let lines = contents.lines().count();
  let words = contents.split_whitespace().count();

  Ok(FileStats {
    filename: filename.to_string(),
    lines,
    words,
    chars,
  })
}
