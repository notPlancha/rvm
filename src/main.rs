mod args;
mod actions;
#[macro_use] mod utils;
mod local_utils;
mod parsing;

use std::path::Path;
use clap::Parser;
use args::{Cli, Action};
use crate::utils::ToAbsolute;

fn main() {
  let args = &Cli::parse();
  // switch functions based on command
  match &args.action {
    Action::Init {rversion, path} =>
      actions::init::main(
        String::from(rversion),
        Path::new(&path),
        args
      ),
    Action::Add {packages, path} => {
      actions::add::main(
        packages.to_owned(),
        Path::new(&path),
        args
      )
    },
    Action::Run {command, path} => {
      dbg!(command);
      dbg!(path);
      todo!()
    }
  }
}

#[allow(non_upper_case_globals)]
#[cfg(test)]
mod tests {
  use crate::local_utils::get_latest_R;
  use crate::parsing::version_parser::{ParseError, Range};
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


    assert_eq!(Version::parse(""), Err(ParseError::InvalidVersion));
    assert_eq!(Version::parse(" "), Err(ParseError::InvalidVersion));
    assert_eq!(Version::parse("Version 1"), Err(ParseError::InvalidVersion));
    assert_eq!(Version::parse("Version-1.2.1"), Err(ParseError::InvalidVersion));
    assert_eq!(p("1"), Version::new(1, 0, 0));
    assert_eq!(p("V1"), Version::new(1, 0, 0));
    assert_eq!(p("1.2"), Version::new(1, 2, 0));
    assert_eq!(p("1+build.1"), Version::new(1, 0, 0).build(Some("build.1")).to_owned());
    assert_eq!(Version::parse("1+windows.1+debian"), Err(ParseError::InvalidVersion));
    assert_eq!(Version::parse("-1.2.3"), Err(ParseError::InvalidVersion));
    assert_eq!(Version::parse("+1.2.3"), Err(ParseError::InvalidVersion));
  }
  #[test]
  #[allow(non_snake_case)]
  fn parse_R_ver() {
    // needs internet connection or will fail
    p(get_latest_R().unwrap().as_str());
  }
  fn p(version: &str) -> Version {
    dbg!(version);
    Version::parse(version).unwrap_or_else(|_| panic!("Failed to parse version: {}", version))
  }
  #[test]
  fn parse_rang() {
    const v: Version = Version::new_const(1, 2, 3);
    assert_eq!(r(">=1.2.3"), Range {
      min: Some(v),
      ..Default::default()
    });
    assert_eq!(r(">1.2.3"), Range {
      min: Some(v.with_patch(4)),
      ..Default::default()
    });
    assert_eq!(r("<1.2.3"), Range {
      max: Some(v),
      ..Default::default()
    });
    assert_eq!(r("<=1.2.3"), Range {
      max: Some(v.with_patch(4)),
      ..Default::default()
    });
    assert_eq!(r("1.2.3"), Range {
      include: vec![v],
      ..Default::default()
    });
    assert_eq!(r("=1.2.3"), Range {
      include: vec![v],
      ..Default::default()
    });
    assert_eq!(r("==1.2.3"), Range {
      include: vec![v],
      ..Default::default()
    });
    assert_eq!(r("== 1.2.3"), Range {
      include: vec![v],
      ..Default::default()
    });
    assert_eq!(r("~1.2.3"), Range {
      min: Some(v),
      max: Some(v.with_minor(3).patch(0).to_owned()),
      ..Default::default()
    });
    assert_eq!(r("~1"), Range {
      min: Some(Version::new(1, 0, 0)),
      max: Some(Version::new(1, 1, 0)),
      ..Default::default()
    });
    assert_eq!(r("~1.2"), Range {
      min: Some(Version::new(1, 2, 0)),
      max: Some(Version::new(1, 3, 0)),
      ..Default::default()
    });
    // assert_eq!(r("~1.2.3-alpha"), Range { // I actually am not sure abt what to do with this TODO
    //   min: Some(v.with_pre_release(Some("alpha"))),
    //   max: Some(v.with_minor(3).patch(0).to_owned()),
    //   ..Default::default()
    // });
    assert_eq!(r("^1.2.3"), Range {
      min: Some(v),
      max: Some(Version::new(2, 0, 0)),
      ..Default::default()
    });
    assert_eq!(r("^ 1.2.3"), Range {
      min: Some(v),
      max: Some(Version::new(2, 0, 0)),
      ..Default::default()
    });

    // multiple versions
    assert_eq!(r(">1.2.3 <1.2.5"), Range {
      min: Some(Version::new(1, 2, 4).to_owned()),
      max: Some(Version::new(1, 2, 5).to_owned()),
      ..Default::default()
    });
    assert_eq!(r("1.2.3 1.2.4"), Range {
      include: vec![Version::new(1, 2, 3).to_owned(), Version::new(1, 2, 4).to_owned()],
      ..Default::default()
    });
    assert_eq!(r(">=1.2.3 <=1.2.5"), Range {
      min: Some(Version::new(1, 2, 3).to_owned()),
      max: Some(Version::new(1, 2, 6).to_owned()),
      ..Default::default()
    });
    assert_eq!(r(">=1.2.3,<=1.2.5"), Range {
      min: Some(Version::new(1, 2, 3).to_owned()),
      max: Some(Version::new(1, 2, 6).to_owned()),
      ..Default::default()
    });
    assert_eq!(r(">=1.2.3, <=1.2.5"), Range {
      min: Some(Version::new(1, 2, 3).to_owned()),
      max: Some(Version::new(1, 2, 6).to_owned()),
      ..Default::default()
    });
    assert_eq!(r(">=1.2.3, <=1.2.5, 1.2.7"), Range {
      min: Some(Version::new(1, 2, 3).to_owned()),
      max: Some(Version::new(1, 2, 6).to_owned()),
      include: vec![Version::new(1, 2, 7).to_owned()],
      ..Default::default()
    });
    assert_eq!(r(">=1.2.3, <=1.2.5, !=1.2.7"), Range {
      min: Some(Version::new(1, 2, 3).to_owned()),
      max: Some(Version::new(1, 2, 6).to_owned()),
      except: vec![Version::new(1, 2, 7).to_owned()],
      ..Default::default()
    });
    //TODO make more extensive tests
  }
  fn r(range: &str) -> Range {
    dbg!(range);
    Range::parse(range).unwrap_or_else(|_| panic!("Failed to parse range: {}", range))
  }
  //TODO test abput comparing versions
  //TODO test about ranging versions
}