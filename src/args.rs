use std::env;
use std::path::PathBuf;
use clap:: {Parser, Subcommand};
use crate::local_utils::curr_dir;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  pub action: Action,
  #[arg(short, long, global = true)] // https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html#flags
  pub verbose: bool, //TODO
  /// if true, don't ask for confirmation
  #[arg(short, long, global = true)]
  pub yes: bool, //TODO
  /// simulate the action, don't actually do it
  #[arg(long="dry-run", global = true)]
  pub dry_run: bool, //TODO
}


#[derive(Subcommand, Debug)]
pub enum Action {
  /// create a new project in the current directory
  Init { //flags
    #[arg(long, short = 'R', default_value = "latest")]
    rversion: String,
    #[arg(long, short, default_value = r".\")]
    path: PathBuf,
  },
  /// add a package to the project
  Add {
    packages: Vec<String>,
    #[arg(long, short, default_value = r".\")]
    path: PathBuf,
  },
  /// run a command in the project
  Run {
    #[arg(long, short)]
    command: String,
    #[arg(long, short, default_value = r".\")]
    path: PathBuf,
  },
}