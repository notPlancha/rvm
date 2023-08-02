mod args;
mod actions;
#[macro_use] mod utils;
mod local_utils;
mod parsing;

use std::path::Path;
use clap::Parser;
use args::{Cli, Action};

fn main() {
  let args = Cli::parse();
  // switch functions based on command
  let action = args.action;
  match action {
    Action::Init {rversion, path} =>
      actions::init::main(
        String::from(rversion),
        Path::new(&path)
      ),
  }
}

#[cfg(test)]
mod tests {
  use crate::parsing::version_parser::ParseError;
  use crate::parsing::version_parser::Version;
  #[test]
  fn parse_ver() {
    let m = Version::new(1, 2, 3);
    assert_eq!(p("1.2.3"), m);
    assert_eq!(p("1.2.3-alpha"),  m.with_pre_release(Some("alpha")));
    assert_eq!(p("1.2.3+build"), m.with_build(Some("build")));
    assert_eq!(p("1.2.3-alpha+build"), m.with_pre_release(Some("alpha")).with_build(Some("build")));
    assert_eq!(p("1.2.3-alpha.1+build.1"), m.with_pre_release(Some("alpha.1")).with_build(Some("build.1")));
    assert_eq!(p("1.2.3.45"), m.with_extra(Some("45")));
    assert_eq!(p("1.2.3.43-alpha.1+build.1"), Version::new_w_extra(1, 2, 3, Some("43"), Some("alpha.1"), Some("build.1")));
    assert_eq!(p("1.2.3.43+windows-alpha.1"), m.with_extra(Some("43")).build(Some("windows")).pre(Some("alpha.1")).to_owned());


    assert_eq!(Version::parse(""), Err(ParseError::Version));
    assert_eq!(Version::parse(" "), Err(ParseError::Version));
    assert_eq!(Version::parse("Version 1"), Err(ParseError::Version));
    assert_eq!(Version::parse("Version-1.2.1"), Err(ParseError::Version));
    assert_eq!(p("1"), Version::new(1, 0, 0));
    assert_eq!(p("V1"), Version::new(1, 0, 0));
    assert_eq!(p("1.2"), Version::new(1, 2, 0));
    assert_eq!(p("1+build.1"), Version::new(1, 0, 0).build(Some("build.1")).to_owned());
    assert_eq!(Version::parse("1+windows.1+debian"), Err(ParseError::Version));
  }
  fn p(version: &str) -> Version {
    dbg!(version);
    Version::parse(version).unwrap_or_else(|_| panic!("Failed to parse version: {}", version))
  }

  //TODO test abput comparing versions
  //TODO test about parsing ranges
  //TODO test about ranging versions
}