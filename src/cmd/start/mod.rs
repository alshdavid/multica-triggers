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
