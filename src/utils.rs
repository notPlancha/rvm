use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use path_clean::PathClean;
use reqwest::blocking::Response;
use thiserror::Error;

macro_rules! request {
  ($typof:ident, $url:expr) => {{ //2 to make an isolated scope
    let response = CLIENT.$typof($url).send().unwrap(); //TODO maybe change unwrap here
    if response.status() != 200{
      return Err(response.status());
    }
    response //returns this
  }};
}
// https://georgik.rocks/how-to-download-binary-file-in-rust-by-reqwest/
pub fn response_to_file_path(folder: PathBuf, name: String, response: Response) -> Result<PathBuf, ResponseToFileError> {
  let filepath = folder.join(name);
  let mut file = File::create(&filepath)?;
  let mut content = io::Cursor::new(response.bytes()?);
  io::copy(&mut content, &mut file)?;
  Ok(filepath)
}

#[derive(Error, Debug)]
pub enum ResponseToFileError {
  #[error("error in creating file")]
  Io(#[from] io::Error),
  #[error("error in response file")]
  Reqwest(#[from] reqwest::Error),
}

pub trait ToAbsolute {
  fn to_absolute(&self) -> PathBuf;
}
impl ToAbsolute for PathBuf {
  fn to_absolute(&self) -> Self {
    return if self.is_relative() {
      std::env::current_dir().unwrap().join(self).clean()
    } else {
      self.clean()
    }
  }
}

impl ToAbsolute for Path {
  fn to_absolute(&self) -> PathBuf {
    return if self.is_relative() {
      std::env::current_dir().unwrap().join(self).clean()
    } else {
      self.clean()
    }
  }
}