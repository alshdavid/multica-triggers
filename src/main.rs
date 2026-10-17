mod cmd;
mod platform;

use clap::Parser;
use clap::Subcommand;

#[derive(Debug, Parser)]
struct Command {
  #[clap(subcommand)]
  command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
  /// Update the binary to the latest release
  #[clap(name = "update", visible_aliases = ["up", "u"])]
  Update(cmd::update::UpdateCommand),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  match Command::parse().command {
    Commands::Update(args) => cmd::update::main(args).await,
  }
}
