use serde::de;
use serde::{Deserialize, Serialize};
use toml::value::Table;

use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TomlSettings {
    #[serde(default = "bool::default")]
    pub create_flake: bool,
    #[serde(default = "bool::default")]
    pub create_overlay: bool,
    #[serde(default = "bool::default")]
    pub create_package: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings() {
        let test_str = r#"
        [settings]
        create_flake = false
        create_overlay = true
        create_package = true
        "#;

        let test_str2 = r#"
        [settings]
        create_flake = true
        "#;

        let manifest = crate::package::TomlManifest::from_str(test_str).unwrap();
        assert!(manifest.github.is_none());
        assert!(manifest.openvsx.is_none());
        assert!(manifest.vsmarketplace.is_none());

        let manifest_settings = *manifest.settings.unwrap();
        assert!(!manifest_settings.create_flake);
        assert!(manifest_settings.create_overlay);
        assert!(manifest_settings.create_package);

        let manifest2 = crate::package::TomlManifest::from_str(test_str2).unwrap();
        assert!(manifest2.github.is_none());
        assert!(manifest2.openvsx.is_none());
        assert!(manifest2.vsmarketplace.is_none());

        let manifest_settings2 = *manifest2.settings.unwrap();
        assert!(manifest_settings2.create_flake);
        assert!(!manifest_settings2.create_overlay);
        assert!(!manifest_settings2.create_package);
    }
}
