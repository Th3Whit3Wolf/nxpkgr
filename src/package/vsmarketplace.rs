use serde::de;
use serde::{Deserialize, Serialize};
use toml::value::Table;

use std::fmt;

use super::config::Overlay;

const DEFAULT_OVERLAY: Overlay = Overlay::VSCode;
fn default_overlay() -> Option<Overlay> {
    Some(DEFAULT_OVERLAY)
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct DetailedTomlVSCodeMarketPlacePackage {
    src: String,
    pin: Option<String>,
    extract: Option<String>,
    #[serde(default = "default_overlay")]
    overlay: Option<Overlay>,
    passthru: Option<Table>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(untagged)]
pub enum TomlVSCodeMarketPlacePackage {
    /// In the simple format, only a unique identifier is specified, eg.
    /// `extension = "<unique.id>"`
    Simple(String),
    /// The simple format is equivalent to a detailed dependency
    /// specifying only a unique identifier, eg.
    /// `extension = { source = "<unique.id>" }`
    Detailed(DetailedTomlVSCodeMarketPlacePackage),
}

impl<'de> de::Deserialize<'de> for TomlVSCodeMarketPlacePackage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct TomlVSCodeMarketPlacePackageVisitor;

        impl<'de> de::Visitor<'de> for TomlVSCodeMarketPlacePackageVisitor {
            type Value = TomlVSCodeMarketPlacePackage;

            fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(
                    "a source string like \"redhat.vscode-xml\" or a \
                     detailed dependency like { source = \"redhat.vscode-xml\" }",
                )
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(TomlVSCodeMarketPlacePackage::Simple(s.to_owned()))
            }

            fn visit_map<V>(self, map: V) -> Result<Self::Value, V::Error>
            where
                V: de::MapAccess<'de>,
            {
                let mvd = de::value::MapAccessDeserializer::new(map);
                DetailedTomlVSCodeMarketPlacePackage::deserialize(mvd)
                    .map(TomlVSCodeMarketPlacePackage::Detailed)
            }
        }

        deserializer.deserialize_any(TomlVSCodeMarketPlacePackageVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_vsmarketplace() {
        let test_str = r#"
		[vsmarketplace]
        dart = "Dart-Code.dart-code"
        gitlab = {src = "GitLab.gitlab-workflow", pin = "3.28.1", passthru = { license = "mit", homepage = "https://open-vsx.org/extension/GitLab/gitlab-workflow", description = "GitLab VSCode integration" } }
		"#;

        let manifest_vsmarketplace = crate::package::TomlManifest::from_str(test_str)
            .unwrap()
            .vsmarketplace
            .unwrap();

        match manifest_vsmarketplace.get("dart").unwrap() {
            TomlVSCodeMarketPlacePackage::Simple(s) => {
                assert_eq!(s, &String::from("Dart-Code.dart-code"))
            }
            TomlVSCodeMarketPlacePackage::Detailed(_) => assert!(false),
        }

        match manifest_vsmarketplace.get("gitlab").unwrap() {
            TomlVSCodeMarketPlacePackage::Simple(_) => assert!(false),
            TomlVSCodeMarketPlacePackage::Detailed(detailed) => {
                assert_eq!(&detailed.src, "GitLab.gitlab-workflow");
                assert_eq!(detailed.pin.as_ref().unwrap(), &String::from("3.28.1"));
                assert!(&detailed.extract.is_none());

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
                match &detailed.overlay.as_ref().unwrap() {
                    Overlay::VSCode => assert!(true),
                    Overlay::Vim => assert!(false),
                    Overlay::None => assert!(false),
                }
            }
        }
    }
}
