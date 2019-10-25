use async_std::fs::File;
use async_std::io::prelude::*;
use async_std::io::BufReader;
use futures::future::join_all;
use serde_derive::Serialize;
use std::fmt;

/// Structure representing the results of a file analysis.
#[derive(Serialize)]
pub struct FileStats {
  /// Number of lines in the file. Based on \n and \r\n
  pub lines: Option<usize>,
  /// Number of words in the file. Based on the Unicode Derived Core Property White_Space
  pub words: Option<usize>,
  /// Number of characters in the file. Based on the Unicode Scalar Value
  pub chars: Option<usize>,
}

impl fmt::Display for FileStats {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    // NOTE: intentional spacing here.
    // Always at least one is reported
    // Last one includes a space too, so removing one from filename print
    write!(
      f,
      "{}{}{}",
      match self.lines {
        Some(lines) => format!("{} ", lines),
        None => String::new(),
      },
      match self.words {
        Some(words) => format!("{} ", words),
        None => String::new(),
      },
      match self.chars {
        Some(chars) => format!("{} ", chars),
        None => String::new(),
      },
    )
  }
}

#[derive(Copy, Clone, Debug)]
pub struct AnalysisOptions {
  pub lines: bool,
  pub words: bool,
  pub chars: bool,
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
      // NOTE: intentional spacing here, as stats leave a trailing space
      NamedOutput::Success { filename, stats } => write!(f, "{}{}", stats, filename),
      NamedOutput::Error { filename, error } => write!(f, "{} {}", error, filename),
    }
  }
}

/// Analyse a single string, returning FileStats
pub fn analyse_string(contents: &str, options: AnalysisOptions) -> FileStats {
  FileStats {
    lines: if options.lines {
      Some(contents.lines().count())
    } else {
      None
    },
    words: if options.words {
      Some(contents.split_whitespace().count())
    } else {
      None
    },
    chars: if options.chars {
      Some(contents.chars().count())
    } else {
      None
    },
  }
}

/// Runs a file analysis on the given filename path.
/// Returns a NamedOutput structure, with the filename and
/// either results or error
pub async fn analyse_file(filename: &str, options: AnalysisOptions) -> NamedOutput {
  let file = match File::open(filename).await {
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
  match buf_reader.read_to_string(&mut contents).await {
    Ok(_bytes) => NamedOutput::Success {
      filename: filename.to_string(),
      stats: analyse_string(&contents, options),
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
pub async fn analyse_files(filenames: &[&str], options: AnalysisOptions) -> Vec<NamedOutput> {
  let iter = filenames
    .iter()
    .map(|filename| analyse_file(filename, options));
  join_all(iter).await
}
