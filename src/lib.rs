use serde_derive::Serialize;
use std::fmt;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use rayon::prelude::*;

/// Structure representing the results of a file analysis.
#[derive(Serialize)]
pub struct FileStats {
  /// Number of lines in the file
  pub lines: usize,
  /// Number of words in the file. TODO: this needs to be strictly defined!
  pub words: usize,
  /// Number of characters in the file. TODO: this needs to be strictly defined!
  pub chars: usize,
}

impl fmt::Display for FileStats {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{} {} {}", self.lines, self.words, self.chars)
  }
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum NamedOutput {
  Success { filename: String, stats: FileStats },
  Error { filename: String, error: String },
}

/// Display as String for CLI use
impl fmt::Display for NamedOutput {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match &*self {
      NamedOutput::Success { filename, stats } => write!(
        f,
        "{} {} {} {}",
        stats.lines, stats.words, stats.chars, filename
      ),
      NamedOutput::Error { filename, error } => write!(f, "{} {}", error, filename),
    }
  }
}

/// Analyse a single string, returning FileStats
pub fn analyse_string(contents: &str) -> FileStats {
  let chars = contents.chars().count();
  let lines = contents.lines().count();
  let words = contents.split_whitespace().count();

  FileStats {
    lines,
    words,
    chars,
  }
}

/// Runs a file analysis on the given filename path.
/// Returns a NamedOutput structure, with the filename and
/// either results or error
pub fn analyse_file(filename: &str) -> NamedOutput {
  let file = match File::open(filename) {
    Err(e) => {
      return NamedOutput::Error {
        filename: filename.to_string(),
        error: e.to_string(),
      }
    }
    Ok(f) => f,
  };

  let mut buf_reader = BufReader::new(file);
  let mut contents = String::new();
  match buf_reader.read_to_string(&mut contents) {
    Ok(_bytes) => NamedOutput::Success {
      filename: filename.to_string(),
      stats: analyse_string(&contents),
    },
    Err(e) => NamedOutput::Error {
      filename: filename.to_string(),
      error: e.to_string(),
    },
  }
}

/// Runs a file analysis on the given list of path buffers.
/// Returns a NamedOutput structure, with the filename and
/// either results or error
pub fn analyse_files(filenames: &[&str]) -> Vec<NamedOutput> {
  filenames
    .par_iter()
    .map(|filename| analyse_file(filename))
    .collect()
}
