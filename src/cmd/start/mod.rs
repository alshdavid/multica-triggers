mod handler_context;
mod handlers;
use std::sync::Arc;

use uhttp::Router;
use uhttp::Server;

use crate::CommandConfig;

use self::handler_context::HandlerContext;

#[derive(Debug, clap::Parser)]
pub struct StartCommand {
  /// Address to bind the server to
  #[clap(long, env = "SERVER_HOST", default_value = "0.0.0.0", hide_env_values = true)]
  pub address: String,

  /// Port to bind the server to
  #[clap(long, env = "SERVER_PORT", default_value_t = 5200, hide_env_values = true)]
  pub port: u16,

  /// Base URL of the Multica instance
  #[clap(long, env = "MULTICA_URL", hide_env_values = true)]
  pub multica_url: String,

  /// Authentication token for the Multica API
  #[clap(long, env = "MULTICA_TOKEN", hide_env_values = true)]
  pub multica_token: String,

  /// Multica workspace identifier
  #[clap(long, env = "MULTICA_WORKSPACE_ID", hide_env_values = true)]
  pub multica_workspace_id: String,

  /// Multica project identifier
  #[clap(long, env = "MULTICA_PROJECT_ID", hide_env_values = true)]
  pub multica_project_id: String,

  /// Multica agent identifier
  #[clap(long, env = "MULTICA_AGENT", hide_env_values = true)]
  pub multica_agent: String,

  /// Authentication token for the GitHub API
  #[clap(long, env = "GITHUB_TOKEN", hide_env_values = true)]
  pub github_token: String,

  /// GitHub account slug (owner/organization)
  #[clap(long, env = "GITHUB_ACCOUNT_SLUG", hide_env_values = true)]
  pub github_account_slug: String,

  /// Label applied to GitHub issues
  #[clap(long, env = "GITHUB_ISSUE_LABEL", hide_env_values = true)]
  pub github_issue_label: String,
}

pub async fn main(config: CommandConfig, args: StartCommand) -> anyhow::Result<()> {
  let bind = format!("{}:{}", args.address, args.port);
  println!("Listening on: http://{}", bind);
  
  let mut app = Router::new(HandlerContext{
    config: Arc::new(config),
    args: Arc::new(args)
  });

  app.any("/triggers/github", handlers::triggers_github_post);

  Server::builder()
    .handler(app.handler())
    .listen(bind)
    .await?;

  Ok(())
}
