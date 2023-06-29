use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use duct::cmd;
use lazy_static::lazy_static;
use path_clean::PathClean;
use reqwest::{StatusCode};
use reqwest::blocking::{Response};
use thiserror::Error;
use tl::queryselector::iterable::QueryIterable;


pub fn install_version(mut path: PathBuf, version: String) {
  path = path.to_absolute();
  let path_str = path.to_str().unwrap();
  let installer = download_R(version, None).unwrap();
  cmd!(installer,
    format!(r"/dir={}\", path_str),
    "/verysilent",
    "/mergetasks=!desktopicon",
    "/currentuser",
  ).run().unwrap();
  println!("R installed in {}", path_str);
}

macro_rules! request {
  ($typof:ident, $url:expr) => {{ //2 to make an isolated scope
    let response = CLIENT.$typof($url).send().unwrap(); //TODO change unwrap here
    if response.status() != 200{
      return Err(response.status());
    }
    response //returns this
  }};
}

#[allow(non_snake_case)]
pub fn download_R(version: String, dest: Option<PathBuf>) -> Result<PathBuf, StatusCode>{
  /// returns the path for the  exe installer
  /// dest is the Windows temp folder if is None
  //TODO change to be possible to change CRAN
  let url: &String = &format!("https://cran.r-project.org/bin/windows/base/old/{}/R-{}-win.exe", version, version);
  request!(head, url); //head request to check if the file exists
  let response: Response = request!(get, url); //get request to download the file
  let destination = dest.unwrap_or(std::env::temp_dir());
  let filename = format!("R-{}-win.exe", version);
  let file = response_to_file(destination, filename, response).unwrap(); // TODO change unwrap here
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

pub fn get_latest() -> Result<String, StatusCode> {
  /// returns the latest version of R using release.html
  /// Example html
  /// <html>
  /// <head>
  /// <META HTTP-EQUIV="Refresh" CONTENT="0; URL=R-4.3.1-win.exe">
  /// <body></body>
  /// TODO it's returning the exe too
  let url = "https://cran.r-project.org/bin/windows/base/release.html";
  let response = request!(get, url);
  let body = response.text().expect("Failed to get body");
  let dom = tl::parse(&body, tl::ParserOptions::default()).expect("Failed to parse body");
  let parser = dom.parser();
  let meta = dom.query_selector("META")
    .expect("Failed to do query")
    .next()
    .expect("failed to parse query")
    .get(parser)
    .expect("failed to get node after parsing")
    .as_tag()
    .expect("failed to cast node to tag");
  let content: &tl::Bytes = meta
    .attributes()
    .get("CONTENT")
    .expect("failed to get attribute")
    .expect("no atribute value found");
  let version = content
    .try_as_utf8_str()
    .expect("failed to parse bytes as string")
    .split("-")
    .nth(1)
    .expect("failed to get version");
  Ok(String::from(version))
}

//lazy static for the client
lazy_static!{
  static ref CLIENT: reqwest::blocking::Client = reqwest::blocking::Client::new();
}

pub trait ToAbsolute {fn to_absolute(&self) -> PathBuf;}
impl ToAbsolute for PathBuf {
  fn to_absolute(&self) -> PathBuf {
    return if self.is_relative() {
      std::env::current_dir().unwrap().join(self).clean()
    } else {
      self.clean()
    }
  }
}