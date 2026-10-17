use std::sync::Arc;

use super::StartCommand;
use crate::CommandConfig;

#[derive(Clone)]
pub struct HandlerContext {
  pub config: Arc<CommandConfig>,
  #[allow(dead_code)]
  pub args: Arc<StartCommand>,
}
