extern crate wordcrab;
use wordcrab::NamedOutput::Error;
use wordcrab::NamedOutput::Success;
use wordcrab::*;

pub static ANALYSIS_OPTIONS_LWC: AnalysisOptions = AnalysisOptions {
  lines: true,
  words: true,
  chars: true,
};

pub static ANALYSIS_OPTIONS_L: AnalysisOptions = AnalysisOptions {
  lines: true,
  words: false,
  chars: false,
};

pub static ANALYSIS_OPTIONS_W: AnalysisOptions = AnalysisOptions {
  lines: false,
  words: true,
  chars: false,
};

pub static ANALYSIS_OPTIONS_C: AnalysisOptions = AnalysisOptions {
  lines: false,
  words: false,
  chars: true,
};

pub static ANALYSIS_OPTIONS_LW: AnalysisOptions = AnalysisOptions {
  lines: true,
  words: true,
  chars: false,
};

pub static ANALYSIS_OPTIONS_LC: AnalysisOptions = AnalysisOptions {
  lines: true,
  words: false,
  chars: true,
};

pub static ANALYSIS_OPTIONS_WC: AnalysisOptions = AnalysisOptions {
  lines: false,
  words: true,
  chars: true,
};

#[test]
fn analysis_options_all() {
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

#[test]
fn analysis_options_l() {
  let named_output = analyse_file("tests/content/seitseman_veljesta.txt", ANALYSIS_OPTIONS_L);
  match named_output {
    Success { filename, stats } => {
      println!("{}", filename);
      assert_eq!(stats.lines, Some(13681));
      assert_eq!(stats.words, None);
      assert_eq!(stats.chars, None);
    }
    Error { filename: _, error } => {
      panic!(error);
    }
  }
}

#[test]
fn analysis_options_w() {
  let named_output = analyse_file("tests/content/seitseman_veljesta.txt", ANALYSIS_OPTIONS_W);
  match named_output {
    Success { filename, stats } => {
      println!("{}", filename);
      assert_eq!(stats.lines, None);
      assert_eq!(stats.words, Some(84192));
      assert_eq!(stats.chars, None);
    }
    Error { filename: _, error } => {
      panic!(error);
    }
  }
}

#[test]
fn analysis_options_c() {
  let named_output = analyse_file("tests/content/seitseman_veljesta.txt", ANALYSIS_OPTIONS_C);
  match named_output {
    Success { filename, stats } => {
      println!("{}", filename);
      assert_eq!(stats.lines, None);
      assert_eq!(stats.words, None);
      assert_eq!(stats.chars, Some(661759));
    }
    Error { filename: _, error } => {
      panic!(error);
    }
  }
}

#[test]
fn analysis_options_lw() {
  let named_output = analyse_file("tests/content/seitseman_veljesta.txt", ANALYSIS_OPTIONS_LW);
  match named_output {
    Success { filename, stats } => {
      println!("{}", filename);
      assert_eq!(stats.lines, Some(13681));
      assert_eq!(stats.words, Some(84192));
      assert_eq!(stats.chars, None);
    }
    Error { filename: _, error } => {
      panic!(error);
    }
  }
}

#[test]
fn analysis_options_lc() {
  let named_output = analyse_file("tests/content/seitseman_veljesta.txt", ANALYSIS_OPTIONS_LC);
  match named_output {
    Success { filename, stats } => {
      println!("{}", filename);
      assert_eq!(stats.lines, Some(13681));
      assert_eq!(stats.words, None);
      assert_eq!(stats.chars, Some(661759));
    }
    Error { filename: _, error } => {
      panic!(error);
    }
  }
}

#[test]
fn analysis_options_wc() {
  let named_output = analyse_file("tests/content/seitseman_veljesta.txt", ANALYSIS_OPTIONS_WC);
  match named_output {
    Success { filename, stats } => {
      println!("{}", filename);
      assert_eq!(stats.lines, None);
      assert_eq!(stats.words, Some(84192));
      assert_eq!(stats.chars, Some(661759));
    }
    Error { filename: _, error } => {
      panic!(error);
    }
  }
}
