use std::iter::Enumerate;
use std::ptr::eq;
use std::str::{Chars, FromStr};
use thiserror::Error;

struct Version {
  major: u32,
  minor: u32,
  patch: u32,
  pre_release: Option<String>,
  build: Option<String>,
}

impl Default for Version {
  fn default() -> Self {
    Self {
      major: 0,
      minor: 0,
      patch: 0,
      pre_release: None,
      build: None,
    }
  }
}

// https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=5a5060e5715aff667d6a263d09fcc690
macro_rules! next_or_return {
  ($var:expr, $ret:expr) => {
    match $iter.next() {
      Some(c) => c,
      None => return Ok($ret),
    }
  };
}

impl Version {
  fn parse(version: String) -> Result<Self, ParseError> {
    let version = version.trim();
    Self::parse_enume(&mut version.chars())
  }
  fn parse_enume(iter: &mut Chars) -> Result<Self, ParseError> {
    // iterator because I wanna continue from the last position and let it continue after
    // https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=fb1ef427abec058c41ecf770313369e4
    //region check if empty and check for v
    let mut curr_char: char = iter.next().ok_or(ParseError::Empty)?;
    if matches!(first, "v" | "V") {
      curr = iter.next().ok_or(ParseError::Empty)?;
    }
    if !matches!(first, "0".."9") {
      //can be also a dot if it's .5.1 for example but this can be a mistake so it's better to not implement it
      return Err(ParseError::Invalid);
    }
    //endregion
    //region main loop
    let major = String::new();
    let minor = String::new();
    let patch = String::new();
    let mut current_ver_part: &String = &major;
    let mut extra_dot = false;
    // https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=1961ff4d2fad5b785603657be4652806
    while let Some(c) = iter.next() {
      curr_char = c;
      if c.is_digit(10){
        *current_ver_part.push(c);
      } else if c == "." {
        // change the current string
        // https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=203544ae269f81d34e5b2292ffb1d0d1
        if eq(current_ver_part, &major) {
          current_ver_part = &minor;
        } else if eq(current_ver_part, &minor) {
          current_ver_part = &patch;
        } else {
          extra_dot = true;
          break;
        }
      } else if matches!(c, "-" | "+") {
        break;
      } else {
        return Err(ParseError::Parse);
      }
    }
    let mut ret = Version {
      major: major.parse()?,
      minor: minor.parse()?,
      patch: patch.parse()?,
      pre_release: None,
      build: None,
    };
    //endregion
    //region pre_release
    if curr_char == "-" {
      let mut pre_release = String::new();
      //get what's in front of the -, until the + or non alphanumeric (except a dot)
      loop {
        curr_char = next_or_return!(iter, ret);
        if curr_char == "." || curr_char.is_alphanumeric() {
          pre_release.push(c);
        } else {
          break;
        }
      }
      ret.pre_release = Some(pre_release);
    }
    //endregion
    //region build
    if matches!(curr_char, "." | "+") {
      let mut build = String::new();
      //get what's in front of the +, until non alphanumeric (except a dot)
      loop {
        curr_char = next_or_return!(iter, ret);
        if curr_char == "." || curr_char.is_alphanumeric() {
          build.push(c);
        } else {
          break;
        }
      }
    }
    //endregion
    Ok(ret)
  }
}

impl FromStr for Version {
  type Err = ParseError;

  fn from_str(s: String) -> Result<Self, Self::Err> {
    Self::parse(s)
  }
}

// to derive error
#[derive(Error, Debug)]
enum ParseError {
  #[error("error in parsing version")]
  Parse,
  #[error("not number in major, minor or patch")]
  NotNumber(#[from] std::num::ParseIntError),
  #[error("Invalid version")]
  Invalid,
  #[error("Empty version")]
  Empty
}

struct Range {
  min: Option<Version>,
  max: Option<Version>,
  exceptions: Vec<Version>,
  exceptions_range: Vec<Range>,
}

impl Range {
  fn parse(range: &str) -> Result<Self, ParseError> {

  }
}

#[derive(Error, Debug)]
enum RangeParseError {
  #[error("error in parsing range")]
  Parse(#[from] ParseError),
  #[error("Invalid range")]
  Invalid,
}

impl Version {
  fn in_range(&self, range: Range) -> bool {
    panic!("not implemented yet") //TODO
  }
}