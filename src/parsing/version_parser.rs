#[macro_use] extern crate lalrpop_util;

use std::str::FromStr;
use lalrpop_util::lalrpop_mod;
use semver::Op;
use thiserror::Error;

lalrpop_mod!(pub version_parser);

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
  extra_version: Option<String>,
  pre_release: Option<String>,
  build: Option<String>,
}

impl Version {
  //noinspection RsUnresolvedReference
  fn parse(version: &str) -> Result<Self, ParseError> {
    let version = version_parser::versionParser::new().parse(version).unwrap_or_else(|_| return Err(ParseError::Version)); //TODO check if 1.0.0. parses well
    Ok(version)
  }
  fn new(
    major: u32,
    minor: Option<u32>,
    patch: Option<u32>,
    extra_version: Option<String>, //1.1.0.1.5 < 1.1.0.1.6, 1.1.0.1.5 > 1.1.0, 1.1.0.0.0 > 1.1.0
    pre_release: Option<String>, //1.1.0-rc.1 < 1.1.0-rc.2, 1.1.0-rc.1 < 1.1.0
    build: Option<String> //1.1.0+build.1 = 1.1.0+build.2, 1.1.0+build.1 = 1.1.0
  ) -> Self {
    Self {
      major,
      minor: minor.unwrap_or(0),
      patch: patch.unwrap_or(0),
      extra_version,
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
    todo!()
  }
}