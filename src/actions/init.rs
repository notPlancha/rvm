use std::fs;
use std::path::{Path, PathBuf};
use path_clean::PathClean;
use crate::utils;
use crate::utils::download_R;

pub fn main(mut rversion: String, path: &Path) {//TODO
  // TODO make verbose
  if rversion == "latest" || rversion == "release" {
    rversion = utils::get_latest().unwrap(); //TODO change unrwap here
  }
  utils::install_version(path.join(r".\env\"), rversion);

  let toml_path = path.join("Renv.toml");
}