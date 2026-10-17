use std::sync::Arc;

use super::StartCommand;

#[derive(Clone)]
pub struct HandlerContext {
  pub args: Arc<StartCommand>
}