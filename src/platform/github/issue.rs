use serde::Deserialize;

/// A GitHub issue as returned by the REST API.
#[derive(Debug, Deserialize)]
pub struct GithubIssue {
  pub number: u64,
  pub title: String,
  pub body: Option<String>,
  pub html_url: String,
  pub repository_url: String,
}

impl GithubIssue {
  /// Repository slug (`owner/name`) the issue belongs to.
  pub fn repo_name(&self) -> String {
    self
      .repository_url
      .replace("https://api.github.com/repos/", "")
  }

  /// REST API URL for the issue itself.
  pub fn api_url(&self) -> String {
    format!("{}/issues/{}", self.repository_url, self.number)
  }
}
