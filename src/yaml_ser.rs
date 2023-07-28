use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use thiserror::Error;

fn write_yaml(env: Env, path: PathBuf) -> Result<(), YamlError> {
  let file = File::create(path)?;
  serde_yaml::to_writer(file, &env)?;
  Ok(())
}

fn read_yaml(path: PathBuf) -> Result<Env, YamlError> {
  let file = File::open(path)?;
  let env: Env = serde_yaml::from_reader(file)?;
  Ok(env)
}

#[derive(Error, Debug)]
enum YamlError {
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
    Self {
      name: Some("My project".to_owned()),
      description: Some("My project description".to_owned()),
      r#type: Some(ProjectType::default()),
      version: None,
      rversion: Rversion {
        accepted: ">=4.3, <4.4".to_string(), //TODO change to dynamic?
        used: "4.3.1".to_string(), //TODO change to dynamic?
        was_latest: None, //TODO change to dynamic?
        tested_on: None
      },
      dependencies: HashMap::from([
        ("pak".to_owned(), "^0.5.1".to_owned())
      ])
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct Rversion {
  pub accepted: String,
  pub used: String,
  pub was_latest: Option<bool>,
  pub tested_on: Option<Vec<String>>
}

#[derive(Default, Serialize, Deserialize)]
pub enum ProjectType {
  Package,
  Jupyter,
  #[default]
  Project
}

