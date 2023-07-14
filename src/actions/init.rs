use std::fs;
use std::path::{Path, PathBuf};
use path_clean::PathClean;
use serde_yaml::Value::String;
use crate::utils;
use crate::utils::download_R;
use crate::yaml_ser::{Env, ProjectType, Rversion};

pub fn main(mut rversion: String, path: &Path) {//TODO
  // TODO make verbose
  let version_is_latest = rversion == "latest" || rversion == "release";
  if version_is_latest {
    rversion = utils::get_latest().unwrap(); //TODO change unrwap here
  }
  utils::install_version(path.join(r".\env\"), rversion);

  let yaml_path = path.join("Renv.yaml");
}