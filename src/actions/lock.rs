use std::path::{Path, PathBuf};
use std::slice::RSplit;
use clap::builder::Str;
use peg::str::LineCol;
use thiserror::Error;
use crate::parsing::grammer::Dependency;
use crate::parsing::version_parser::{Range, Version};
use crate::parsing::grammer::the_parser::{parse_dependencies};
use crate::utils::{ToAbsolute};

#[allow(non_snake_case)]
#[derive(Default)]
pub struct Package {
  pub name: String,
  pub priority: Priority,
  pub version: Version,
  pub source: Source,
  pub dependencies: Vec<Dependency>,
  pub Rrange: Range
}

#[derive(Error, Debug)]
enum PackageError {
  #[error("error in reading file")]
  Io(#[from] std::io::Error),
  #[error("error in serializing .yaml file")]
  Serialization, // Custom serde
  #[error("error in parsing")]
  Parsing(#[from] peg::error::ParseError<LineCol>),
  #[error("error in parsing version or range")]
  Version(#[from] crate::parsing::version_parser::ParseError),
  #[error("error in description keys")]
  Description
}

impl Package {
  pub fn getCurrentPackages(env_path: &Path) -> Vec<Package> {
    let library_path = env_path.join(r".\library\").to_absolute();
    let mut ret: Vec<Package> = Vec::new();
    // for all folders in the path
    for entry in library_path.read_dir().unwrap() {
      let entry = entry.unwrap();
      let path = entry.path();
      // if the folder is a package
      if path.join("DESCRIPTION").exists() {
        let package = Package::from_description(path.join("DESCRIPTION")).unwrap();
        ret.push(package);
      }
    }
    ret
  }

  pub fn from_description(path: PathBuf) -> Result<Package, peg::error::ParseError<LineCol>> {
    // get line by line
    let file = std::fs::read_to_string(path).unwrap();
    let mut ret = Package::default();
    let mut deps : Vec<Dependency> = vec![];
    for line in file.lines() {
      //check if starts with needed
      if line.starts_with("Package:") {
        ret.name = line.split_once(":").unwrap().1.trim().to_string();
      }
      else if line.starts_with("Priority:") {
        ret.priority = match line.split_once(":").unwrap().1.trim() {
          "base" => Priority::Base,
          "recommended" => Priority::Recommended,
          _ => Priority::None
        }
      }
      else if line.starts_with("Version:") {
        ret.version = Version::parse(line.split_once(":").unwrap().1.trim()).unwrap();
      }
      else if line.starts_with("Depends:") || line.starts_with("Imports:") {
        let mut these_deps = parse_dependencies(line.split_once(":").unwrap().1.trim())?;
        // remove and collect Rversion
        let mut R_dep = these_deps.iter().position(|x| x.name == "R");
        if R_dep.is_some() {
          ret.Rrange = these_deps.remove(R_dep.unwrap()).range;
        }
        deps.extend(these_deps);
      }
    }
    ret.dependencies = deps;
    Ok(ret)
  }
}

pub fn get_current_packages(env_path: &Path) -> Vec<Package> {
  Package::getCurrentPackages(env_path)
}

#[derive(Default)]
pub enum Priority {
  Base,
  Recommended,
  #[default] None
}

#[derive(Default)]
pub enum Source {
  CRAN,
  Github,
  Gitlab,
  Bioconductor,
  Url,
  Local,
  #[default] Unknown
}