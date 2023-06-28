mod args;
mod actions;
mod utils;

use std::path::PathBuf;
use clap::Parser;
use args::{Cli, Action};

fn main() {
  let args = Cli::parse();
  // switch functions based on command
  let action = args.action;
  match action {
    Action::Init {rversion, path} =>
      actions::init::main(
        String::from(rversion),
        PathBuf::from(path)
      ),
  }
}