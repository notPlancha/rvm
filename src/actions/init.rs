use std::fs;
use std::path::{Path, PathBuf};
use path_clean::PathClean;
use crate::utils;
use crate::utils::download_R;

pub fn main(mut rversion: String, mut path: PathBuf) {//TODO
  if path.is_relative() {
    path = std::env::current_dir().unwrap().join(path).clean(); //TODO don't unrwap?
  }
  let version: String;
  if rversion == "latest" || rversion == "release" {
    rversion = utils::get_latest().unwrap(); //TODO change unrwap here
  }
  //let installed = utils::install_version(&path.unwrap_or(), version);
}