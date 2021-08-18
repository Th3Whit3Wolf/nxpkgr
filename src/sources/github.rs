use color_eyre::{
    eyre::{eyre, Report, Result, WrapErr},
    Section,
};

use reqwest::header::{HeaderMap, HeaderValue, ACCEPT};
use serde::{Deserialize, Serialize};
use tokio::{runtime::Handle, task};

use crate::nix::{NixLicense, NixPackage, NixPackageMeta};

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitHubRepo {
    pub id: u64,
    pub node_id: String,
    pub name: String,
    pub full_name: String,
    pub private: bool,
    pub owner: GitHubOwner,
    pub html_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub fork: bool,
    pub url: String,
    pub forks_url: String,
    pub keys_url: String,
    pub collaborators_url: String,
    pub teams_url: String,
    pub hooks_url: String,
    pub issue_events_url: String,
    pub events_url: String,
    pub assignees_url: String,
    pub branches_url: String,
    pub tags_url: String,
    pub blobs_url: String,
    pub git_tags_url: String,
    pub git_refs_url: String,
    pub trees_url: String,
    pub statuses_url: String,
    pub languages_url: String,
    pub stargazers_url: String,
    pub contributors_url: String,
    pub subscribers_url: String,
    pub subscription_url: String,
    pub commits_url: String,
    pub git_commits_url: String,
    pub comments_url: String,
    pub issue_comment_url: String,
    pub contents_url: String,
    pub compare_url: String,
    pub merges_url: String,
    pub archive_url: String,
    pub downloads_url: String,
    pub issues_url: String,
    pub pulls_url: String,
    pub milestones_url: String,
    pub notifications_url: String,
    pub labels_url: String,
    pub releases_url: String,
    pub deployments_url: String,
    pub created_at: String,
    pub updated_at: String,
    pub pushed_at: String,
    pub git_url: String,
    pub ssh_url: String,
    pub clone_url: String,
    pub svn_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<String>,
    pub size: u64,
    pub stargazers_count: u64,
    pub watchers_count: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    pub has_issues: bool,
    pub has_projects: bool,
    pub has_downloads: bool,
    pub has_wiki: bool,
    pub has_pages: bool,
    pub forks_count: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mirror_url: Option<bool>,
    pub archived: bool,
    pub disabled: bool,
    pub open_issues_count: u64,
    pub license: GitHubLicense,
    pub forks: u64,
    pub open_issues: u64,
    pub watchers: u64,
    pub default_branch: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temp_clone_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<GitHubOrganization>,
    pub network_count: u64,
    pub subscribers_count: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitHubOwner {
    pub login: String,
    pub id: u64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub site_admin: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitHubLicense {
    pub key: String,
    pub name: String,
    pub spdx_id: String,
    pub url: String,
    pub node_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitHubOrganization {
    pub login: String,
    pub id: u64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub site_admin: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitHubTags {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<GitHubTag>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitHubTag {
    pub name: String,
    pub commit: GitHubCommit,
    pub zipball_url: String,
    pub tarball_url: String,
    pub node_id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GitHubCommit {
    pub sha: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitHubBranches {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<GitHubTag>>,
}

/// For all branches
/// /repos/{owner}/{repo}/branches
/// For specific branch
/// /repos/{owner}/{repo}/branches/{branch}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitHubBranch {
    pub name: String,
    pub commit: GitHubCommit,
    pub protected: bool,
    pub protection: GitHubProtection,
    protection_url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitHubProtection {
    pub required_status_checks: GitHubRequiredStatusChecks,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitHubRequiredStatusChecks {
    pub enforcement_level: String,
    pub contexts: Vec<String>,
}

/// Path is optional
/// get /repos/{owner}/{repo}/contents/{path}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitHubContents {
    pub contents: Vec<GitHubContent>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitHubContent {
    #[serde(rename = "type")]
    pub type_type: String,
    pub encoding: String,
    pub size: u64,
    pub name: String,
    pub path: String,
    pub content: String,
    pub sha: String,
    pub url: String,
    pub git_url: String,
    pub html_url: String,
    pub download_url: String,
    #[serde(rename = "_links")]
    pub links: GitHubContentLinks,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitHubContentLinks {
    pub git: String,
    #[serde(rename = "self")]
    pub self_type: String,
    pub html: String,
}

/// /repos/{owner}/{repo}/releases
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitHubReleases {
    pub releases: Vec<GitHubRelease>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitHubRelease {
    pub url: String,
    pub html_url: String,
    pub assets_url: String,
    pub upload_url: String,
    pub tarball_url: String,
    pub zipball_url: String,
    pub id: u64,
    pub node_id: String,
    pub tag_name: String,
    pub target_commitish: String,
    pub name: String,
    pub body: String,
    pub draft: bool,
    pub prerelease: bool,
    pub created_at: String,
    pub published_at: String,
    pub author: GitHubReleaseAuthor,
    pub assets: Vec<GitHubReleaseAssets>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitHubReleaseAuthor {
    pub login: String,
    pub id: u64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub type_type: String,
    pub site_admin: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitHubReleaseAssets {
    pub url: String,
    pub browser_download_url: String,
    pub id: u64,
    pub node_id: String,
    pub name: String,
    pub label: String,
    pub state: String,
    pub content_type: String,
    pub size: u64,
    pub download_count: u64,
    pub created_at: String,
    pub updated_at: String,
    pub uploader: GitHubReleaseAssetsUploader,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GitHubReleaseAssetsUploader {
    pub login: String,
    pub id: u64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub type_type: String,
    pub site_admin: bool,
}

impl GitHubRepo {
    pub async fn get(github_owner_repo: &str) -> Result<GitHubRepo> {
        let owner_repo: Vec<&str> = github_owner_repo.split('/').collect();
        let owner = owner_repo[0];
        let repo = owner_repo[1];
        let url = format!("https://api.github.com/repos/{}/{}", owner, repo);

        let response: GitHubRepo = reqwest::Client::new()
            .get(url)
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "nxpkgr")
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }
}
