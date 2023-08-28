use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::local_utils::get_latest;
use crate::parsing::version_parser::{ParseError, Range, Version};


pub fn write_yaml(env: Env, path: PathBuf) -> Result<(), YamlError> {
  let file = File::create(path)?;
  serde_yaml::to_writer(file, &env)?;
  Ok(())
}

pub fn read_yaml(path: PathBuf) -> Result<Env, YamlError> {
  let file = File::open(path)?;
  let env: Env = serde_yaml::from_reader(file)?;
  Ok(env)
}

#[derive(Error, Debug)]
pub enum YamlError {
  #[error("error in creating/reading file")]
  Io(#[from] std::io::Error),
  #[error("error in parsing .yaml file")]
  Serde(#[from] serde_yaml::Error),
}

#[derive(Serialize, Deserialize)]
pub struct Env {
  pub name: Option<String>,
  pub description: Option<String>,
  pub r#type: Option<ProjectType>,
  pub version: Option<String>,
  pub rversion: Rversion,
  pub dependencies: HashMap<String, String>
}

impl Default for Env {
  fn default() -> Self {
    let latest = get_latest().unwrap_or_default();
    Self {
      name: Some("My project".to_owned()),
      description: Some("My project description".to_owned()),
      r#type: Some(ProjectType::default()),
      version: Some("1.0.0".to_string()),
      rversion: Rversion::from_str(&latest).unwrap_or_else(|err| panic!("Failed to parse version: {:?}", err)),
      dependencies: HashMap::from([
        ("pak".to_owned(), "^0.5.1".to_owned())
      ])
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct Rversion {
  pub accepted: Range,
  pub pinned: Version,
}

impl FromStr for Rversion {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    //like above
    let accepted = Range::parse(format!("^{}", s).as_str())?;
    let pinned = Version::parse(s)?;
    Ok(Self {
      accepted,
      pinned
    })
  }
}

#[derive(Default, Serialize, Deserialize)]
pub enum ProjectType {
  Package,
  Jupyter,
  #[default]
  Project
}