use std::time::Duration;

#[derive(Debug, clap::Parser)]
pub struct StartCommand {
  /// Task(s) to start
  #[clap(long, short = 't')]
  pub task: Vec<String>,

  /// Interval between runs (e.g. 30s, 5m, 1h)
  #[clap(long, value_parser = humantime::parse_duration, default_value = "30s")]
  pub interval: Duration,
}

pub async fn main(args: StartCommand) -> anyhow::Result<()> {
  loop {
    let _ = &args.task;

    tokio::time::sleep(args.interval).await;
  }
}
