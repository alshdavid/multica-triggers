use serde::Deserialize;
use uhttp::HandlerResponse;
use uhttp::StatusCode;

use crate::cmd::start::handler_context::HandlerContext;

#[derive(Debug, Deserialize)]
pub struct GithubWebhookPayload {
  pub action: Option<String>,
  pub issue: Option<GithubWebhookIssue>,
  pub label: Option<GithubWebhookLabel>,
}

#[derive(Debug, Deserialize)]
pub struct GithubWebhookIssue {
  pub html_url: Option<String>,
  pub labels: Option<Vec<GithubWebhookLabel>>,
}

#[derive(Debug, Deserialize)]
pub struct GithubWebhookLabel {
  pub name: Option<String>,
}

pub async fn triggers_github_post(
  mut req: uhttp::Request,
  res: uhttp::Response,
  HandlerContext{ args, ..}: HandlerContext,
) -> uhttp::Result<HandlerResponse> {
  let body = uhttp::body::json::<GithubWebhookPayload>(&mut req.body).await?;

  let is_labeled = body.action.as_deref() == Some("labeled");
  let is_sentinel = body
    .label
    .as_ref()
    .and_then(|label| label.name.as_deref())
    == Some("sentinel");

  if is_labeled
    && is_sentinel
    && let Some(html_url) = body.issue.as_ref().and_then(|issue| issue.html_url.as_deref())
  {
    println!("{html_url}");
  }

  res.status(StatusCode::NO_CONTENT).body(vec![])
}