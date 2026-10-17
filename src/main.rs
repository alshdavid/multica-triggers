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

  /// Manage GitHub issues
  #[clap(name = "github_issues", visible_aliases = ["github-issues", "gh-issues"])]
  GithubIssues(cmd::github_issues::GithubIssuesCommand),

  /// Start the service
  #[clap(name = "start")]
  Start(cmd::start::StartCommand),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  match Command::parse().command {
    Commands::Update(args) => cmd::update::main(args).await,
    Commands::GithubIssues(args) => cmd::github_issues::main(args).await,
    Commands::Start(args) => cmd::start::main(args).await,
  }
}
