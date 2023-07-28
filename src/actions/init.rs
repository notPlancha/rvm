use std::path::Path;
use std::string::String;
use crate::local_utils::{get_latest, install_version};

pub fn main(mut rversion: String, path: &Path) {//TODO
  // TODO make verbose
  let version_is_latest = rversion == "latest" || rversion == "release";
  if version_is_latest {
    rversion = get_latest().unwrap(); //TODO change unrwap here
  }
  install_version(path.join(r".\env\"), rversion);

  let yaml_path = path.join("Renv.yaml");
}