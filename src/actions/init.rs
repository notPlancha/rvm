use std::collections::HashMap;
use std::hash::Hash;
use std::path::Path;
use std::str::FromStr;
use std::string::String;
use cli_prompts::DisplayPrompt;
use cli_prompts::prompts::Confirmation;
use serde::Serialize;
use crate::actions::confirmation_style;
use crate::actions::lock::{get_current_packages, Package, Priority};
use crate::args::Cli;
use crate::local_utils::{get_latest_R, install_version};
use crate::parsing::version_parser;
use crate::parsing::version_parser::Range;
use crate::parsing::yaml_ser::{Env, write_yaml};
use crate::utils::ToAbsolute;

pub fn main(mut rversion: String, path: &Path, options : &Cli) {
  // TODO make verbose
  // TODO make dry run
  create_folder_if_needed(path);
  let env_path = path.join(r".\env\");
  let yaml_path = path.join(r".\Renv.yaml");
  //region check if paths are empty
  let env_exists = env_path.exists() && env_path.read_dir().unwrap().next().is_some();
  let yaml_exists = yaml_path.exists();
  if env_exists || yaml_exists {
    if options.yes {
      println!("The path seems to have a project started already, overwriting env and/or yaml");
    } else {
      let can_overwrite = Confirmation::new(format!("The path seems to have a project started already, do you want to overwrite env and/or yaml?"))
        .default_positive(true)
        .style(confirmation_style())
        .display()
        .unwrap_or_else(|err| panic!("Aborted: {:?}", err));
      if !can_overwrite {
        panic!("Aborted: User chose not to overwrite")
      }
    }
    // delete the env folder and yaml file
    if env_exists {
      std::fs::remove_dir_all(&env_path).unwrap_or_else(|err| panic!("Failed to remove env folder: {:?}", err));
    }
    if yaml_exists {
      std::fs::remove_file(&yaml_path).unwrap_or_else(|err| panic!("Failed to remove yaml file: {:?}", err));
    }
  }
  //endregion
  create_folder_if_needed(&env_path);
  let version_is_latest = rversion == "latest" || rversion == "release"; //TODO move this out of this function me thinks
  if version_is_latest {
    rversion = get_latest_R().unwrap(); //TODO change unrwap here
  } else {
    // check if version is valid
    version_parser::Version::parse(&rversion).unwrap_or_else(|err| panic!("Failed to parse version: {:?}", err));
  }
  install_version(env_path.clone(), &rversion);
  //region build env and yaml file
  let curr_packs:Vec<Package> = get_current_packages(&env_path);
  // because this is a fresh install, this is only going to be base and reccomended
  // base no need to specify, reccomended can
  // so remove base from vector
  let mut depends: HashMap<String, Range> = HashMap::new();
  for pack in curr_packs {
    match pack.priority {
      Priority::Recommended => {depends.insert(pack.name, pack.Rrange); ()}, // to return nothing
      _ => ()
    }
  }
  let env = Env {
    rversion: Range::from_str(&rversion).unwrap(), // should be alerady checked b4
    dependencies: depends,
    ..Default::default()
  };
  //endregion
  write_yaml(env, yaml_path).unwrap_or_else(|err| panic!("Failed to write yaml: {:?}", err)); //TODO version recieving null
}

fn create_folder_if_needed(path: &Path) {
  if !path.exists() {
    std::fs::create_dir_all(path).unwrap_or_else(|err| panic!("Failed to create path {}: {:?}", path.to_absolute().display(), err));
  }
}