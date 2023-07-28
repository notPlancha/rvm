mod args;
mod actions;
#[macro_use] mod utils;
mod local_utils;
mod yaml_ser;


use std::path::Path;
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
        Path::new(&path)
      ),
  }
}