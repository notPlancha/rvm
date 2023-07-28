use std::str::FromStr;
use semver::Op;
use thiserror::Error;

#[derive(Error, Debug)]
enum ParseError {
  #[error("error in parsing version")]
  Version,
  #[error("error in parsing range")]
  Range,
}

pub struct Version {
  major: u32,
  minor: u32,
  patch: u32,
  pre_release: Option<String>,
  build: Option<String>,
}

impl Version {
  fn parse(version: &str) -> Self {
    panic!("TODO")
  }
  fn new(
    major: u32,
    minor: Option<u32>,
    patch: Option<u32>,
    pre_release: Option<String>,
    build: Option<String>
  ) -> Self {
    Self {
      major,
      minor: minor.unwrap_or(0),
      patch: patch.unwrap_or(0),
      pre_release,
      build,
    }
  }
}

struct Range {
  min: Option<Version>,
  max: Option<Version>,
  except_ranges: Vec<Range>,
  except_versions: Vec<Version>,
}

impl Range {
  fn contains(&self, version: Version) -> bool {
    panic!("TODO")
  }
}