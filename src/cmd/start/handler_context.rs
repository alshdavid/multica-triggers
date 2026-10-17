use std::sync::Arc;

use super::StartCommand;
use crate::CommandConfig;

#[derive(Clone)]
pub struct HandlerContext {
  pub config: Arc<CommandConfig>,
  pub args: Arc<StartCommand>,
}
