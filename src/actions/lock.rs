use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use clap::builder::Str;
use thiserror::Error;
use crate::parsing::grammer::Dependency;
use crate::parsing::version_parser::{ParseError, Range, Version};
use crate::parsing::grammer::the_parser::{parse_dependencies};

#[allow(non_snake_case)]
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
  #[error("error in parsing .yaml file")]
  Serde(#[from] serde_yaml::Error),
  #[error("error in parsing")]
  Parsing(#[from] ParseError),
  #[error("error in description keys")]
  Description
}

fn get_key(tree: &BTreeMap<String, str>, key: &str) -> Result<&str, PackageError> {
  tree.get(key).ok_or(PackageError::Description)
}

impl Package {
  pub fn getCurrentPackages(env_path: &Path) -> Vec<Package> {
    let library_path = env_path.join(r"library\");
    let mut ret: Vec<Package> = Vec::new();
    // for all folders in the path
    for entry in library_path.read_dir().unwrap() {
      let entry = entry.unwrap();
      let path = entry.path();
      // if the folder is a package
      if path.join("DESCRIPTION").exists() {
        let package = Package::from_description(path.join("DESCRIPTION"));
        ret.push(package);
      }

    }
    ret

  }

  pub fn from_description(path: PathBuf) -> Package {
    let buff = std::fs::read_to_string(path).unwrap();
    Package::from_description_buff(&buff).unwrap()
  }


  fn from_description_buff(buff: &str) -> Result<Package, PackageError> {
    let desc: BTreeMap<String, str> = serde_yaml::from_str(buff)?;
    let name = get_key(&desc, "Package")?.to_owned();
    let version = Version::parse(get_key(&desc, "Version")?)?;
    let priority = match desc.get("Priority") {
      Some("base") => Priority::Base,
      Some("recommended") => Priority::Recommended,
      _ => Priority::None
    };
    let mut deps: Vec<Dependency> = Vec::new();
    let deps_depends = desc.get("Depends");
    if deps_depends.is_some() {
      deps.extend(parse_dependencies(deps_depends.unwrap())?);
    }
    let deps_imports = desc.get("Imports");
    if deps_imports.is_some() {
      deps.extend(parse_dependencies(deps_imports.unwrap())?);
    }
    // extract R version from deps (has name R) and remove from deps
    let mut Rrange = Range::default();
    let mut new_deps: Vec<Dependency> = Vec::new();
    for dep in deps {
      if dep.name == "R" {
        Rrange = dep.range;
      } else {
        new_deps.push(dep);
      }
    }
    Ok(Package {
      name,
      priority,
      version,
      source: Source::Unknown,
      dependencies: new_deps,
      Rrange
    })
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
enum Source {
  CRAN,
  Github,
  Gitlab,
  Bioconductor,
  Url,
  Local,
  #[default] Unknown
}