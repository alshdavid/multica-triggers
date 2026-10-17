use reqwest::Client;
use reqwest::StatusCode;
use serde::Deserialize;

use super::MulticaIssue;

/// Parameters for creating a Multica issue.
#[derive(Debug)]
pub struct CreateIssue<'a> {
  pub title: &'a str,
  pub description: &'a str,
  pub project_id: &'a str,
  pub agent: &'a str,
}

/// Outcome of a create-issue request.
#[derive(Debug)]
pub enum CreateIssueOutcome {
  /// The issue was created.
  Created(MulticaIssue),
  /// An equivalent issue is already active and was returned instead.
  Duplicate(MulticaIssue),
}

/// Create an issue, reporting when an equivalent issue is already active.
pub async fn create_issue(
  base_url: &str,
  token: &str,
  workspace_id: &str,
  issue: CreateIssue<'_>,
) -> anyhow::Result<CreateIssueOutcome> {
  let body = serde_json::json!({
    "title": issue.title,
    "description": issue.description,
    "project_id": issue.project_id,
    "assignee_type": "agent",
    "assignee_id": issue.agent,
  });

  let response = Client::new()
    .post(format!("{base_url}/api/issues"))
    .header("Content-Type", "application/json")
    .header("Authorization", format!("Bearer {token}"))
    .header("X-Workspace-ID", workspace_id)
    .json(&body)
    .send()
    .await?;

  let status = response.status();

  if status == StatusCode::CONFLICT {
    let data: DuplicateResponse = response.json().await?;
    if data.code.as_deref() == Some("active_duplicate_issue")
      && let Some(issue) = data.issue
    {
      return Ok(CreateIssueOutcome::Duplicate(issue));
    }
    anyhow::bail!("Multica API error {status}: active_duplicate_issue");
  }

  if !status.is_success() {
    let text = response.text().await.unwrap_or_default();
    anyhow::bail!("Multica API error {status}: {text}");
  }

  let issue: MulticaIssue = response.json().await?;
  Ok(CreateIssueOutcome::Created(issue))
}

#[derive(Debug, Deserialize)]
struct DuplicateResponse {
  code: Option<String>,
  issue: Option<MulticaIssue>,
}
