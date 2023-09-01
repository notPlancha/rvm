use std::path::PathBuf;
use duct::cmd;
use lazy_static::lazy_static;
use reqwest::blocking::Response;
use reqwest::StatusCode;
use crate::utils::{response_to_file_path, ToAbsolute};
use cli_prompts::{
  DisplayPrompt,
  prompts::{Confirmation, Input},
  style::{Color, ConfirmationStyle, Formatting, InputStyle, LabelStyle},
};
pub fn install_version(mut path: PathBuf, version: &String) {
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

#[allow(non_snake_case)]
pub fn download_R(version: &String, dest: Option<PathBuf>) -> Result<PathBuf, StatusCode>{
  // returns the path for the  exe installer
  // dest is the Windows temp folder if is None
  //TODO change to be possible to change CRAN
  let url: &String = &format!("https://cran.r-project.org/bin/windows/base/old/{}/R-{}-win.exe", version, version); //TODO change it to toggle between archived and not (20 versions behind thye change url)
  request!(head, url); //head request to check if the file exists
  let response: Response = request!(get, url); //get request to download the file
  let destination = dest.unwrap_or(std::env::temp_dir());
  let filename = format!("R-{}-win.exe", version);
  let file = response_to_file_path(destination, filename, response).unwrap(); // TODO change unwrap here
  Ok(file)
}


#[allow(non_snake_case)]
pub fn get_latest_R() -> Result<String, StatusCode> {
  // returns the latest version of R using release.html
  // Example html
  // <html>
  // <head>
  // <META HTTP-EQUIV="Refresh" CONTENT="0; URL=R-4.3.1-win.exe">
  // <body></body>
  // TODO it's returning the exe too
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

// lazy static is here for singleton-like patterns
lazy_static!{
  // http client
  static ref CLIENT: reqwest::blocking::Client = reqwest::blocking::Client::new();
}

pub fn curr_dir() -> PathBuf {
  // Here it can panic because it should be getting anywhere
  std::env::current_dir().unwrap_or_else(|err| panic!("Failed to get current dir: {:?}", err))
}