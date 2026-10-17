use std::sync::Arc;

use crate::CommandConfig;

use super::StartCommand;

#[derive(Clone)]
pub struct HandlerContext {
  pub config: Arc<CommandConfig>,
  pub args: Arc<StartCommand>
}