use clap:: {Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  pub action: Action,
  #[arg(short, long)] // https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html#flags
  pub verbose: bool, //TODO
  /// if true, don't ask for confirmation
  #[arg(short, long)]
  pub yes: bool, //TODO
  /// simulate the action, don't actually do it
  #[arg(long="dry-run")]
  pub dry_run: bool, //TODO
}


#[derive(Subcommand, Debug)]
pub enum Action {
  /// create a new project in the current directory
  Init { //flags
    #[arg(long, short = 'R', default_value = "latest")]
    rversion: String,
    #[arg(long, short, default_value = r".\")]
    path: String,
  }
}