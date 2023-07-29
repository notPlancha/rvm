#[macro_use] extern crate lalrpop_util;

use std::cmp::Ordering;
use std::str::FromStr;
use lalrpop_util::lalrpop_mod;
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
    let version: Self = version_parser::versionParser::new().parse(version).unwrap_or_else(|_| return Err(ParseError::Version));
    Ok(version)
  }
  fn new(
    major: u32,
    minor: u32,
    patch: u32,
    //1.1.0.1.5 < 1.1.0.1.6, 1.1.0.1.5 > 1.1.0, 1.1.0.0.0 > 1.1.0
    extra_version: Option<String>,
    // 1.1.0-rc.1 < 1.1.0-rc.2, 1-a < 1-b, 1.1.0-rc.1 <= 1.1.0
    // é menor que ele mas no range é igual, tipo uma espécie de epsilon
    // isto é porque o range espera-se que por exemplo >= 1.0, < 2.0 não inclua 2.0-alpha
    // embora tecnicamente inclui pq é antes
    // ainda assim quando for para comparar versões, 2.0-alpha é menor que 2.0 na mesma (por exemplo pra atualizar)
    pre_release: Option<String>,
    //1.1.0+build.1 = 1.1.0+build.2, 1.1.0+build.1 = 1.1.0
    build: Option<String>
  ) -> Self {
    Self {
      major,
      minor,
      patch,
      extra_version,
      pre_release,
      build,
    }
  }
  //could be a cool macro
  //maybe remove if not used anywhere
  fn with_major(&self, major: u32) -> Self {
    Self {
      major,
      minor: self.minor,
      patch: self.patch,
      extra_version: self.extra_version.clone(),
      pre_release: self.pre_release.clone(),
      build: self.build.clone(),
    }
  }
  fn with_minor(&self, minor: u32) -> Self {
    Self {
      major: self.major,
      minor,
      patch: self.patch,
      extra_version: self.extra_version.clone(),
      pre_release: self.pre_release.clone(),
      build: self.build.clone(),
    }
  }
  fn with_patch(&self, patch: u32) -> Self {
    Self {
      major: self.major,
      minor: self.minor,
      patch,
      extra_version: self.extra_version.clone(),
      pre_release: self.pre_release.clone(),
      build: self.build.clone(),
    }
  }
  fn with_extra_version(&self, extra_version: Option<String>) -> Self {
    Self {
      major: self.major,
      minor: self.minor,
      patch: self.patch,
      extra_version,
      pre_release: self.pre_release.clone(),
      build: self.build.clone(),
    }
  }
  fn with_pre_release(&self, pre_release: Option<String>) -> Self {
    Self {
      major: self.major,
      minor: self.minor,
      patch: self.patch,
      extra_version: self.extra_version.clone(),
      pre_release,
      build: self.build.clone(),
    }
  }
  fn with_build(&self, build: Option<String>) -> Self {
    Self {
      major: self.major,
      minor: self.minor,
      patch: self.patch,
      extra_version: self.extra_version.clone(),
      pre_release: self.pre_release.clone(),
      build,
    }
  }
}

impl FromStr for Version {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Self::parse(s)
  }
}

impl Eq for Version {

}

impl PartialEq for Version {
  fn eq(&self, other: &Self) -> bool {
    todo!()
  }
}

impl PartialOrd<Version> for Version {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    todo!()
  }
}

impl Ord for Version {
  fn cmp(&self, other: &Self) -> Ordering {
    todo!()
  }
}

struct Range {
  min: Option<Version>, //inclusive
  max: Option<Version>, //exclusive, because it's hard to go back to the previous version
  except_ranges: Vec<Range>,
  except_versions: Vec<Version>,
}

impl Range {
  fn contains(&self, version: Version) -> bool {
    todo!()
  }
  fn is_any(&self) -> bool { // == is empty
    todo!()
  }
  fn is_valid(&self) -> bool {
    todo!()
  }
  fn is_exact_match(&self) -> bool {
    todo!()
  }
  fn new(
    min: Option<Version>,
    max: Option<Version>,
    except_ranges: Vec<Range>,
    except_versions: Vec<Version>,
  ) -> Self {
    Self {
      min,
      max,
      except_ranges,
      except_versions,
    }
  }
  fn from_ver_vec(ranges: Vec<(Op, Version)>) -> Self {

  }
  fn mixed_vec_to_stand_vec(ranges: Vec<(Op, Version)>) -> Vec<(Op, Version)> {
    // Expand tilde and caret ranges to simple Gt/Lt/Ge/Le ranges
    ranges.into_iter().flat_map(|(op, version)| {
      match op {
        Op::Tilde => Self::tilde_range_to_vec(version),
        Op::Caret => Self::caret_range_to_vec(version),
        _ => vec![(op, version)],
      }
    }).collect::<Vec<_>>()
  }

  fn sort_vec(ranges: Vec<(Op, Version)>) -> Vec<(Op, Version)> {
    // Sort the ranges by version number
    let mut ranges = ranges;
    ranges.sort_by(|(_, a), (_, b)| a.cmp(&b));
    ranges
  }

  //noinspection RsUnresolvedReference
  fn parse(range: &str) -> Result<Self, ParseError> {
    let range: Vec<(Op, Version)>  = version_parser::rangesParser::new().parse(range).unwrap_or_else(|_| return Err(ParseError::Range));
    Ok(Self::from_ver_vec(range))
  }

  fn tilde_range_to_vec(version: Version) -> Vec<(Op, Version)> {
    // ~1.2.3 -> >=1.2.3 <1.3.0
    // ~1.2 -> >=1.2.0 <1.3.0
    // ~1 -> >=1.0.0 <1.1.0, since 1 = 1.0.0
    vec![
      (Op::Ge, version.clone()),
      (Op::Lt, Version::new(version.major, version.minor + 1, 0, None, None, None)),
    ]
  }
  fn caret_range_to_vec(version: Version) -> Vec<(Op, Version)> {
    // ^1.2.3 -> >=1.2.3 <2.0.0
    // ^1.2 -> >=1.2.0 <2.0.0
    // ^1 -> >=1.0.0 <2.0.0, since 1 = 1.0.0
    vec![
      (Op::Ge, version.clone()),
      (Op::Lt, Version::new(version.major + 1, 0, 0, None, None, None)),
    ]
  }
}

pub enum Op {
  Eq,    //==
  Ne,    //!=
  Gt,    //>
  Lt,    //<
  Ge,    //>=
  Le,    //<=
  Tilde, //~
  Caret  //^
}

impl Op {
  fn from_str(op: &str) -> Result<Self, ParseError> {
    match op {
      "==" | "=" | "" => Ok(Self::Eq),
      "!=" => Ok(Self::Ne),
      ">" => Ok(Self::Gt),
      "<" => Ok(Self::Lt),
      ">=" => Ok(Self::Ge),
      "<=" => Ok(Self::Le),
      "~" => Ok(Self::Tilde),
      "^" => Ok(Self::Caret),
      _ => Err(ParseError::Range)
    }
  }
}