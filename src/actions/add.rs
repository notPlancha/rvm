use std::path::Path;
use crate::args::Cli;

pub fn main(packages: Vec<String>, path: &Path, options: &Cli) {
  // parse packages and check if they're valid
  type Responsible = String;
  let dependencies: Vec<String>;

  for package in packages {
    // parse for version

    // check CRAN for it TODO implement github and gitlab (and bioconductor?)
  }

  // install


  // add to yaml
}