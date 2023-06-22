use clap:: {Args as cArgs, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Option<Command>
}


#[derive(Subcommand, Debug)]
pub enum Command {
  /// create a new project in the current directory
  Init { //flags
    #[arg(short = 'R', long = "Rversion", default_value = "latest")]
    rversion: String,
  }
}