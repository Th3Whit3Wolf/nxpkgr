use serde::de;
use serde::{Deserialize, Serialize};
use toml::value::Table;

use std::fmt;

use super::config::Overlay;

impl<'de> de::Deserialize<'de> for GitHubPin {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct GitHubPinVisitor;

        impl<'de> de::Visitor<'de> for GitHubPinVisitor {
            type Value = GitHubPin;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("A colon seperated string")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if let Some((pin_kind, pin_to)) = s.split_once(":") {
                    match pin_kind.trim() {
                        "commit" => Ok(GitHubPin::ToCommit(pin_to.trim().to_owned())),
                        "release" => Ok(GitHubPin::ToRelease(pin_to.trim().to_owned())),
                        "tag" => Ok(GitHubPin::ToTag(pin_to.trim().to_owned())),
                        _ => Err(E::custom("Expected commit:, release:, or tag:")),
                    }
                } else {
                    Err(E::custom("Expected one of (commit,release,tag) followed by a ':' and the commit, release, or tag you would like to pin to"))
                }
            }
        }
        deserializer.deserialize_any(GitHubPinVisitor)
    }
}

impl<'de> de::Deserialize<'de> for GitHubLatest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct GitHubLatestVisitor;

        impl<'de> de::Visitor<'de> for GitHubLatestVisitor {
            type Value = GitHubLatest;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str("A colon seperated string")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if let Some((latest_kind, latest_from)) = s.split_once(":") {
                    match latest_kind.trim() {
                        "branch" => Ok(GitHubLatest::FromBranch(latest_from.trim().to_owned())),
                        "release" => Ok(GitHubLatest::FromRelease(latest_from.trim().to_owned())),
                        _ => Err(E::custom("Expected branch: or release:")),
                    }
                } else {
                    Err(E::custom("Expected one of (branch or release) followed by a ':' and the brnach or release, or tag you would like to get the latest from"))
                }
            }
        }
        deserializer.deserialize_any(GitHubLatestVisitor)
    }
}

impl<'de> de::Deserialize<'de> for TomlGitHubPackage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct TomlGitHubPackageVisitor;

        impl<'de> de::Visitor<'de> for TomlGitHubPackageVisitor {
            type Value = TomlGitHubPackage;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(
                    "a github repository source string like \"Nixos/nixpkgs\" or a \
                     detailed dependency like { version = \"Nixos/nixpkgs\" }",
                )
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(TomlGitHubPackage::Simple(s.to_owned()))
            }

            fn visit_map<V>(self, map: V) -> Result<Self::Value, V::Error>
            where
                V: de::MapAccess<'de>,
            {
                let mvd = de::value::MapAccessDeserializer::new(map);
                DetailedTomlGitHubPackage::deserialize(mvd).map(TomlGitHubPackage::Detailed)
            }
        }

        deserializer.deserialize_any(TomlGitHubPackageVisitor)
    }
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum GitHubLatest {
    FromRelease(String),
    FromBranch(String),
}

#[derive(Debug, Serialize, Clone, PartialEq)]
pub enum GitHubPin {
    ToRelease(String),
    ToCommit(String),
    ToTag(String),
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DetailedTomlGitHubPackage {
    src: String,
    latest: Option<GitHubLatest>,
    pin: Option<GitHubPin>,
    extract: Option<String>,
    passthru: Option<Table>,
    overlay: Option<Overlay>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum TomlGitHubPackage {
    /// In the simple format, only a version is specified, eg.
    /// `package = "<version>"`
    Simple(String),
    /// The simple format is equivalent to a detailed dependency
    /// specifying only a version, eg.
    /// `package = { version = "<version>" }`
    Detailed(DetailedTomlGitHubPackage),
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_github() {
        let test_str = r#"
		[github]
		eww = "elkowar/eww"
		nixos =  {src = "Nixos/nixpkgs",  pin = "commit:13aa00156246eec1043c795a9cd3f09dac6928fa"}
		gitlab = {src = "GitLab.gitlab-workflow", latest = "branch:main", passthru = { license = "mit", homepage = "https://open-vsx.org/extension/GitLab/gitlab-workflow", description = "GitLab VSCode integration" } }

		[github.sled]
		src = "spacejam/sled"
		"#;

        let manifest_github = crate::package::TomlManifest::from_str(test_str)
            .unwrap()
            .github
            .unwrap();

        match manifest_github.get("eww").unwrap() {
            TomlGitHubPackage::Simple(s) => assert_eq!(s, &String::from("elkowar/eww")),
            TomlGitHubPackage::Detailed(_) => assert!(false),
        }

        match manifest_github.get("nixos").unwrap() {
            TomlGitHubPackage::Simple(_) => assert!(false),
            TomlGitHubPackage::Detailed(detailed) => {
                assert_eq!(&detailed.src, "Nixos/nixpkgs");
                assert!(&detailed.latest.is_none());
                assert!(&detailed.extract.is_none());
                assert!(&detailed.passthru.is_none());
                assert!(&detailed.overlay.is_none());
                let pin = detailed.pin.as_ref().unwrap();
                match pin {
                    GitHubPin::ToRelease(_) => assert!(false),
                    GitHubPin::ToCommit(s) => {
                        assert_eq!(s, &String::from("13aa00156246eec1043c795a9cd3f09dac6928fa"))
                    }
                    GitHubPin::ToTag(_) => assert!(false),
                }
            }
        }

        match manifest_github.get("gitlab").unwrap() {
            TomlGitHubPackage::Simple(_) => assert!(false),
            TomlGitHubPackage::Detailed(detailed) => {
                assert_eq!(&detailed.src, "GitLab.gitlab-workflow");
                assert!(&detailed.pin.is_none());
                assert!(&detailed.extract.is_none());
                assert!(&detailed.overlay.is_none());
                let latest = detailed.latest.as_ref().unwrap();
                match latest {
                    GitHubLatest::FromBranch(s) => assert_eq!(s, &String::from("main")),
                    GitHubLatest::FromRelease(_) => assert!(false),
                }
                let pt = &detailed.passthru.as_ref().unwrap();

                assert_eq!(pt.get("license").unwrap().as_str().unwrap(), "mit");
                assert_eq!(
                    pt.get("homepage").unwrap().as_str().unwrap(),
                    "https://open-vsx.org/extension/GitLab/gitlab-workflow"
                );
                assert_eq!(
                    pt.get("description").unwrap().as_str().unwrap(),
                    "GitLab VSCode integration"
                );
            }
        }

        match manifest_github.get("sled").unwrap() {
            TomlGitHubPackage::Simple(_) => assert!(false),
            TomlGitHubPackage::Detailed(detailed) => {
                assert_eq!(&detailed.src, "spacejam/sled");
                assert!(&detailed.latest.is_none());
                assert!(&detailed.pin.is_none());
                assert!(&detailed.extract.is_none());
                assert!(&detailed.passthru.is_none());
                assert!(&detailed.overlay.is_none());
            }
        }
    }
}
