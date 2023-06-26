mod args;
mod actions;
mod utils;

extern crate clap;
use clap::Parser;
use args::{Cli, Action};

fn main() {
  let args = Cli::parse();
  // switch functions based on command
  match args.action {
    Action::Init {..} => actions::init::init(args.action.rversion),
  }
}
