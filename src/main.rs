mod args;
mod commands;
extern crate clap;
use clap::Parser;
use args::Cli;

fn main() {
  let args = Cli::parse();
  // check if it's Init
  println!("{:?}", args.command );
}
