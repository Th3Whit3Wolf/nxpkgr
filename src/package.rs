use serde_derive::{Deserialize, Serialize};
use toml::Value;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Sources {
    #[serde(rename = "vsmarketplace")]
    VSCodeMarketPlace,
    #[serde(rename = "openvsx")]
    OpenVSX,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Package {
    pub name: String,
    pub src: Sources,
}

impl Package {
    pub fn get_packages(toml_file_as_string: String) -> Vec<Package> {
        let mut packages: Vec<Package> = Vec::new();
        let mut tml = toml_file_as_string.parse::<Value>().unwrap();

        if let Some(table) = tml.as_table_mut() {
            for (k, v) in table {
                if v.is_table() {
                    if let Some(t) = v.as_table_mut() {
                        t.insert(String::from("name"), Value::String(k.to_string()));
                    }
                    let package: Package = toml::from_str(&v.to_string()).unwrap();
                    packages.push(package)
                }
            }
        }
        packages
    }
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum PackageKind {
    VscodeExtension {
        publisher: String,
        extension_name: String,
    },
    Other,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct NixPackage {
    pub kind: PackageKind,
    pub name: String,
    pub pname: String,
    pub src: String,
    pub version: String,
    pub sha256: String,
}

/*
meta = with lib; {
    description = "A utility that combines the usability of The Silver Searcher with the raw speed of grep";
    homepage = "https://github.com/BurntSushi/ripgrep";
    license = with licenses; [ unlicense /* or */ mit ];
    maintainers = with maintainers; [ tailhook globin ma27 zowoq ];
    mainProgram = "rg";
  };
*/

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_basic_conf() {
        let toml_str = r#"
        ["roscop.activefileinstatusbar"]
        src = "vsmarketplace"

        ["cometeer.spacemacs"]
        src = "vsmarketplace"

         "#
        .to_string();

        let actual = Package::get_packages(toml_str);

        let expected = vec![
            Package {
                name: "cometeer.spacemacs".to_string(),
                src: Sources::VSCodeMarketPlace,
            },
            Package {
                name: "roscop.activefileinstatusbar".to_string(),
                src: Sources::VSCodeMarketPlace,
            },
        ];
        assert_eq!(expected, actual)
    }
}
