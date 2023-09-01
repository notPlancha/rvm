use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use crate::local_utils::get_latest_R;
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
  pub rversion: Range,
  pub dependencies: HashMap<String, Range>
}

impl Default for Env {
  fn default() -> Self {
    let latest = get_latest_R().unwrap_or_default();
    Self {
      name: Some("My project".to_owned()),
      description: Some("My project description".to_owned()),
      r#type: Some(ProjectType::default()),
      version: Some("1.0.0".to_string()),
      rversion: Range::from_str(format!("^{}", latest).as_str()).unwrap(),
      dependencies: HashMap::new()
    }
  }
}

#[derive(Default, Serialize, Deserialize)]
pub enum ProjectType {
  Package,
  Jupyter,
  #[default] Project
}