use serde::Deserialize;
use uhttp::HandlerResponse;
use uhttp::StatusCode;

use super::super::handler_context::HandlerContext;
use crate::cmd::action_github_issue;
use crate::cmd::action_github_issue::ActionGithubIssueCommand;

#[derive(Debug, Deserialize)]
pub struct GithubWebhookPayload {
  pub action: Option<String>,
  pub issue: Option<GithubWebhookIssue>,
  pub label: Option<GithubWebhookLabel>,
  pub repository: Option<GithubWebhookRepository>,
}

#[derive(Debug, Deserialize)]
pub struct GithubWebhookRepository {
  pub full_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GithubWebhookIssue {
  // pub html_url: Option<String>,
  // pub labels: Option<Vec<GithubWebhookLabel>>,
  pub number: Option<u64>,
}

#[derive(Debug, Deserialize)]
pub struct GithubWebhookLabel {
  pub name: Option<String>,
}

pub async fn triggers_github_post(
  mut req: uhttp::Request,
  res: uhttp::Response,
  HandlerContext { config, .. }: HandlerContext,
) -> uhttp::Result<HandlerResponse> {
  // dbg!(uhttp::body::json::<serde_json::Value>(&mut req.body).await?);

  let body = uhttp::body::json::<GithubWebhookPayload>(&mut req.body).await?;

  let is_labeled = body.action.as_deref() == Some("labeled");
  let is_sentinel =
    body.label.as_ref().and_then(|label| label.name.as_deref()) == Some(config.label.as_str());

  if is_labeled
    && is_sentinel
    && let Some(repository) = body.repository.as_ref()
    && let Some(full_name) = repository.full_name.as_deref()
    && let Some(number) = body.issue.as_ref().and_then(|issue| issue.number)
  {
    tokio::spawn(action_github_issue::main(
      (*config).clone(),
      ActionGithubIssueCommand {
        repo: full_name.to_string(),
        issue: number,
      },
    ));
  }

  res.status(StatusCode::NO_CONTENT).body(vec![])
}
