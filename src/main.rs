mod args;
mod actions;
#[macro_use] mod utils;
mod local_utils;
mod parsing;

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

#[cfg(test)]
mod tests {
  use crate::parsing::version_parser::Version;
  #[test]
  fn parse_ver() {
    assert_eq!(Version::parse("1.2.3"), Ok(Version::new(1, 2, 3, None, None, None)));
  }
}