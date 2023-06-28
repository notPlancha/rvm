use std::path::{Path, PathBuf};
use crate::utils;
use crate::utils::download_R;

pub fn main(rversion: String, path: Option<PathBuf>) {//TODO
  let version: String;
  if rversion == "latest" || rversion == "release" {
    version = utils::get_latest().unwrap(); //TODO change unrwap here
  } else {
    version = rversion;
  }
  dbg!("init");
  let downloaded_to = download_R(version, None).unwrap();
  dbg!(downloaded_to);
}