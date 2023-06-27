mod args;
mod actions;
mod utils;

use clap::Parser;
use args::{Cli, Action};

fn main() {
  let args = Cli::parse();
  // switch functions based on command
  let action = args.action;
  match action {
    Action::Init {rversion, ..} => actions::init::main(String::from("4.3.1"), None), //TODO change to flags
  }

}
