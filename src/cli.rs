use chrono::{Local, NaiveDate};
use clap::Parser;

#[derive(serde::Deserialize)]
pub struct CfgFile {
    gitlab_host: Option<String>,
    gitlab_username: Option<String>,
    gitlab_token: Option<String>,
}

impl CfgFile {
    pub fn to_cli_args(self) -> Vec<(String, String)> {
        let mut args = Vec::new();
        if let Some(host) = self.gitlab_host {
            args.push(("--host".to_string(), host));
        }
        if let Some(username) = self.gitlab_username {
            args.push(("--username".to_string(), username));
        }
        if let Some(token) = self.gitlab_token {
            args.push(("--token".to_string(), token));
        }
        args
    }
}

/// CLI Arguments for `clap`. If not present, the values are taken from
/// environment variables.
#[derive(Parser, Debug)]
#[command(
    version,
    about = "\
Tool to fetch the timelogs from the GitLab API and display them in a helpful
way. Can either be configured via CLI options, environment variables, or by
a `~/.config/gitlab-timelogs/config.toml` file that looks as follows:

gitlab_host = \"gitlab.example.com\"
gitlab_username = \"<user>\"
gitlab_token = \"<token>\""
)]
pub struct CliArgs {
    /// The GitLab host without `https://`. For example `gitlab.example.com`.
    #[arg(long = "host", env)]
    gitlab_host: String,
    /// Your GitLab username.
    #[arg(long = "username", env)]
    gitlab_username: String,
    /// Token with read access to GitLab API. You can get one on
    /// `https://<gitlab_host>/-/user_settings/personal_access_tokens`.
    #[arg(long = "token", env)]
    gitlab_token: String,
    /// Filter for newest date (inclusive). For example `2024-06-30`.
    /// By default, this defaults to today (local time).
    ///
    /// Must be no less than `--after`.
    #[arg(long = "before")]
    gitlab_before: Option<NaiveDate>,
    /// Filter for oldest date (inclusive). For example `2024-06-01`.
    ///
    /// Must be no more than `--before`.
    #[arg(long = "after", default_value = "1970-01-01")]
    gitlab_after: NaiveDate,
}

impl CliArgs {
    pub fn host(&self) -> &str {
        &self.gitlab_host
    }
    pub fn username(&self) -> &str {
        &self.gitlab_username
    }
    pub fn token(&self) -> &str {
        &self.gitlab_token
    }
    pub fn before(&self) -> NaiveDate {
        // This is a bit of a hack, because Clap's default_value_t doesn't seem
        // to work with clap_serde_derive. *sigh*
        self.gitlab_before.unwrap_or(current_date())
    }
    pub fn after(&self) -> NaiveDate {
        self.gitlab_after
    }
}

fn current_date() -> NaiveDate {
    Local::now().naive_local().date()
}
