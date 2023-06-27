use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use reqwest::{StatusCode};
use reqwest::blocking::{Response};
use thiserror::Error;
//
// pub fn get_latest() -> String {
//
// }

pub fn install_version(path: &Path, version: &str) -> bool {
  let args = "/silent /mergetasks=!desktopicon /dir=here";//TODO change this part
  return true;
}

pub fn download_R(version: String, dest: Option<PathBuf>) -> Result<PathBuf, StatusCode>{
  /// returns the path for the  exe installer
  /// dest is the Windows temp folder if is None
  //TODO change to be possible to change CRAN
  let url: &String = &format!("https://cran.r-project.org/bin/windows/base/old/{}/R-{}-win.exe", version, version);
  let client = reqwest::blocking::Client::new();
  let status = client.head(url).send().unwrap().status(); //head request to check if the file exists
  if status != 200 {
    return Err(status);
  }
  let response: Response = client.get(url).send().unwrap();
  let destination = dest.unwrap_or(std::env::temp_dir());
  let filename = format!("R-{}-win.exe", version);
  let file = response_to_file(destination, filename, response).unwrap(); // TODO change unrwap here
  Ok(file)
}

//https://georgik.rocks/how-to-download-binary-file-in-rust-by-reqwest/
fn response_to_file(folder: PathBuf, name: String, response: Response) -> Result<PathBuf, ResponseToFileError> {
  let filepath = folder.join(name);
  let mut file = File::create(&filepath)?;
  let mut content = io::Cursor::new(response.bytes()?);
  io::copy(&mut content, &mut file)?;
  Ok(filepath)
}

#[derive(Error, Debug)]
enum ResponseToFileError {
  #[error("error in creating file")]
  Io(#[from] io::Error),
  #[error("error in response file")]
  Reqwest(#[from] reqwest::Error),
}