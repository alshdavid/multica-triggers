use crate::platform::self_upgrade;
use crate::platform::self_upgrade::TARGET_REPO;
use crate::platform::self_upgrade::UpgradeOptions;
use crate::platform::self_upgrade::UpgradeOutcome;

#[derive(Debug, clap::Parser)]
pub struct UpdateCommand {
  /// Check for an available update without installing it
  #[arg(long)]
  pub check: bool,
}

pub async fn main(args: UpdateCommand) -> anyhow::Result<()> {
  let current_version = env!("CARGO_PKG_VERSION");

  let options = UpgradeOptions {
    target_repo: TARGET_REPO,
    current_version,
  };

  if args.check {
    println!("Checking for updates");

    match self_upgrade::check_for_update(&options).await {
      Ok(Some(version)) => {
        println!("Update available: {version} (current: {current_version})");
      }
      Ok(None) => {
        println!("Already on the latest version ({})", current_version);
      }
      Err(err) => {
        eprintln!("Check failed: {err}");
      }
    }

    return Ok(());
  }

  println!("Starting Self Upgrade");

  let outcome = self_upgrade::try_upgrade(&options).await;

  match outcome {
    Ok(UpgradeOutcome::Skip) => {
      println!("Already on the latest version ({})", current_version);
    }
    Ok(UpgradeOutcome::Success) => {
      println!("Upgrade complete");
    }
    Err(err) => {
      eprintln!("Upgrade failed: {err}");
    }
  }

  Ok(())
}
