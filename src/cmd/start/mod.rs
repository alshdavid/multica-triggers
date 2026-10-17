mod handler_context;
mod handlers;
use std::sync::Arc;

use uhttp::Router;
use uhttp::Server;

use self::handler_context::HandlerContext;

#[derive(Debug, clap::Parser)]
pub struct StartCommand {
  /// Address to bind the server to
  #[clap(long, env = "SERVER_HOST", default_value = "0.0.0.0")]
  pub address: String,

  /// Port to bind the server to
  #[clap(long, env = "SERVER_PORT", default_value_t = 5200)]
  pub port: u16,

  /// Base URL of the Multica instance
  #[clap(long, env = "MULTICA_URL")]
  pub multica_url: String,

  /// Authentication token for the Multica API
  #[clap(long, env = "MULTICA_TOKEN")]
  pub multica_token: String,

  /// Multica workspace identifier
  #[clap(long, env = "MULTICA_WORKSPACE_ID")]
  pub multica_workspace_id: String,

  /// Multica project identifier
  #[clap(long, env = "MULTICA_PROJECT_ID")]
  pub multica_project_id: String,

  /// Multica agent identifier
  #[clap(long, env = "MULTICA_AGENT")]
  pub multica_agent: String,

  /// Authentication token for the GitHub API
  #[clap(long, env = "GITHUB_TOKEN")]
  pub github_token: String,

  /// GitHub account slug (owner/organization)
  #[clap(long, env = "GITHUB_ACCOUNT_SLUG")]
  pub github_account_slug: String,

  /// Label applied to GitHub issues
  #[clap(long, env = "GITHUB_ISSUE_LABEL")]
  pub github_issue_label: String,
}

pub async fn main(args: StartCommand) -> anyhow::Result<()> {
  let bind = format!("{}:{}", args.address, args.port);
  println!("Listening on: http://{}", bind);
  
  let mut app = Router::new(HandlerContext{
    args: Arc::new(args)
  });

  app.any("/triggers/github", handlers::triggers_github_post);

  Server::builder()
    .handler(app.handler())
    .listen(bind)
    .await?;

  Ok(())
}
