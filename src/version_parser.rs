use std::ptr::eq;
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

impl Version {
  fn parse(version: &str) -> Result<Self, ParseError> {
    let version = version.trim();
    // enumerate the string
    let mut chars_enume = version.chars().enumerate();
    //first char
    let mut curr = next_or_err!(chars_enume, ParseError::Empty);

    if matches!(first, "v" | "V") {
      // skip the first character
      curr = next_or_err!(chars_enume, ParseError::Empty);
    }
    if !matches!(first, "0".."9") {
      //can be also a dot if it's .5.1 for example but this can be a mistake so it's better to not implement it
      return Err(ParseError::Invalid);
    }

    let major = String::new();
    let minor = String::new();
    let patch = String::new();
    let mut current: &String = &major;
    let mut extra_dot = false;
    let mut latest_char: &char;
    // https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=1961ff4d2fad5b785603657be4652806
    while let Some((i, c)) = chars_enume.next() {
      latest_char = &c;
      if c.is_digit(10){
        *current.push(c);
      } else if c == "." {
        // change the current string
        // https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=203544ae269f81d34e5b2292ffb1d0d1
        if eq(current, &major) {
          current = &minor;
        } else if eq(current, &minor) {
          current = &patch;
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
    if latest_char == "-" {
      let mut pre_release = String::new();
      //get what's in front of the -, until the +
      while let Some((i, c)) = chars_enume.next() {
        if c == "+" {
          break;
        }
        pre_release.push(c);
      }
      ret.pre_release = Some(pre_release);
    }
    if extra_dot || latest_char == "+" {
      let mut build = String::new();
      //get what's in front of the +
      while let Some((i, c)) = chars_enume.next() {
        build.push(c);
      }
      ret.build = Some(build);
    }
    Ok(ret)
  }
}

// macro for that let Some thing, with the chars and the error message as args
macro_rules! next_or_err {
  ($enume:expr, $err:expr) => {{
    let Some(ret) = $enume.next() else {return Err($err)};
    ret
  }};
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
  Empty,
}

struct Range {
  min: Option<Version>,
  max: Option<Version>,
  exceptions: Vec<Version>,
}

impl Range {
  fn parse(range: &str) -> Result<Self, ParseError> {
    // makes a vector of the versions and its ops, then transforms it into a range

    panic!("not implemented yet") //TODO
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