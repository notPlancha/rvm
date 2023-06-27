use std::path::{Path, PathBuf};
use crate::utils::download_R;

pub fn main(rversion: String, path: Option<PathBuf>) {//TODO
  if rversion == "latest" || rversion == "release" {
    dbg!("init latest");
  }
  dbg!("init");
  let downloaded_to = download_R(rversion, None).unwrap();
  dbg!(downloaded_to);
}