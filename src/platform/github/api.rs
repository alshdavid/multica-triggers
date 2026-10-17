use reqwest::Client;
use reqwest::Method;
use reqwest::RequestBuilder;
use reqwest::Response;
use serde::de::DeserializeOwned;

use super::GithubIssue;

const USER_AGENT: &str = "multica-triggers";
const API_BASE: &str = "https://api.github.com";
const API_VERSION: &str = "2022-11-28";

/// Fetch the full context of a single issue.
pub async fn get_issue(
  token: &str,
  repo: &str,
  number: u64,
) -> anyhow::Result<GithubIssue> {
  let response = request(
    token,
    Method::GET,
    format!("{API_BASE}/repos/{repo}/issues/{number}"),
  )
  .send()
  .await?;

  parse(response).await
}

/// Add a label to an issue.
pub async fn add_label(
  token: &str,
  issue: &GithubIssue,
  label: &str,
) -> anyhow::Result<()> {
  let response = request(token, Method::POST, format!("{}/labels", issue.api_url()))
    .json(&serde_json::json!({ "labels": [label] }))
    .send()
    .await?;

  ensure_success(response).await?;
  Ok(())
}

fn request(
  token: &str,
  method: Method,
  url: String,
) -> RequestBuilder {
  Client::new()
    .request(method, url)
    .header("Accept", "application/vnd.github+json")
    .header("X-GitHub-Api-Version", API_VERSION)
    .header("Authorization", format!("Bearer {token}"))
    .header("User-Agent", USER_AGENT)
}

async fn ensure_success(response: Response) -> anyhow::Result<Response> {
  let status = response.status();
  if !status.is_success() {
    let text = response.text().await.unwrap_or_default();
    anyhow::bail!("GitHub API error {status}: {text}");
  }
  Ok(response)
}

async fn parse<T: DeserializeOwned>(response: Response) -> anyhow::Result<T> {
  let response = ensure_success(response).await?;
  Ok(response.json().await?)
}
