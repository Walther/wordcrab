use serde_derive::Serialize;
use std::fmt;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

/// Structure representing the results of a file analysis.
#[derive(Serialize)]
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

impl fmt::Display for FileStats {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{} {} {} {}",
      self.lines, self.words, self.chars, self.filename
    )
  }
}

/// Structure representing an error result of a file analysis.
#[derive(Serialize)]
pub struct FileStatsError {
  /// String representation of the filename given as the input, including possible relative path
  pub filename: String,
  /// Error message. TODO: keep `std::io::Error` here, and somehow derive the to_string?
  pub error: String,
}

impl fmt::Display for FileStatsError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{} {}", self.error, self.filename)
  }
}

#[derive(Serialize)]
#[serde(untagged)]
// Enum that can either be a FileStats or a FileStatsError
pub enum FileStatsOutput {
  Success(FileStats),
  Error(FileStatsError),
}

impl fmt::Display for FileStatsOutput {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match &*self {
      FileStatsOutput::Success(file_stats) => FileStats::fmt(&file_stats, f),
      FileStatsOutput::Error(file_stats_error) => FileStatsError::fmt(&file_stats_error, f),
    }
  }
}

/// Runs a file analysis on the given filename path.
/// Returns a FileStats structure representing the results,
/// or a FileStatsError containing the filename and the error.
pub fn analyse_file(filename: &str) -> FileStatsOutput {
  let file = match File::open(filename) {
    Ok(file) => file,
    Err(e) => {
      return FileStatsOutput::Error(FileStatsError {
        filename: filename.to_string(),
        error: e.to_string(),
      });
    }
  };
  let mut buf_reader = BufReader::new(file);
  let mut contents = String::new();
  match buf_reader.read_to_string(&mut contents) {
    Ok(_bytes) => {
      let chars = contents.chars().count();
      let lines = contents.lines().count();
      let words = contents.split_whitespace().count();

      FileStatsOutput::Success(FileStats {
        filename: filename.to_string(),
        lines,
        words,
        chars,
      })
    }
    Err(e) => {
      return FileStatsOutput::Error(FileStatsError {
        filename: filename.to_string(),
        error: e.to_string(),
      })
    }
  }
}
