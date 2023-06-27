use clap:: {Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  pub action: Action
}


#[derive(Subcommand, Debug)]
pub enum Action {
  /// create a new project in the current directory
  Init { //flags
    #[arg(short = 'R', long = "Rversion", default_value = "latest")]
    rversion: String,
  }
}