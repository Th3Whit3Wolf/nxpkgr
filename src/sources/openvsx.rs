use serde_derive::{Deserialize, Serialize};
//use serde::{Deserialize, Serialize};

use anyhow::Result;

use std::collections::HashMap;

use crate::{
    license::NixLicense,
    package::{NixPackage, NixPackageMeta, PackageKind},
    sources::{get_hash, get_long_description},
};

use tokio::{runtime::Handle, task};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct OpenVSXExtensionResponse {
    pub namespaceUrl: String,
    pub reviewsUrl: String,
    pub files: OpenVSXExtensionResponseFile,
    pub name: String,
    pub namespace: String,
    pub version: String,
    pub publishedBy: OpenVSXExtensionResponsePublishedBy,
    pub verified: bool,
    pub unrelatedPublisher: bool,
    pub namespaceAccess: String,
    pub allVersions: HashMap<String, String>,
    pub averageRating: Option<f64>,
    pub downloadCount: Option<u64>,
    pub reviewCount: u64,
    pub versionAlias: Vec<String>,
    pub timestamp: String,
    pub preview: bool,
    pub displayName: String,
    pub description: String,
    pub engines: HashMap<String, String>,
    pub categories: Vec<String>,
    pub extensionKind: Vec<String>,
    pub tags: Vec<String>,
    pub license: String,
    pub repository: String,
    pub bugs: String,
    pub galleryColor: Option<String>,
    pub galleryTheme: Option<String>,
    pub dependencies: Vec<String>,
    pub bundledExtensions: Vec<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct OpenVSXExtensionResponseFile {
    changelog: String,
    download: String,
    readme: String,
    icon: String,
    manifest: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct OpenVSXExtensionResponsePublishedBy {
    loginName: String,
    fullName: String,
    avatarUrl: String,
    homepage: String,
    provider: String,
}

impl OpenVSXExtensionResponse {
    pub async fn new(extension: &str) -> Result<Self> {
        let split_ext: Vec<&str> = extension.split('.').collect();
        let author = split_ext[0];
        let ext_name = split_ext[1];
        let url = format!("https://open-vsx.org/api/{}/{}", author, ext_name);

        let response: OpenVSXExtensionResponse =
            reqwest::Client::new().get(url).send().await?.json().await?;

        Ok(response)
    }
}

impl From<OpenVSXExtensionResponse> for NixPackage {
    fn from(ext: OpenVSXExtensionResponse) -> Self {
        let publisher: String = ext.namespace.to_string();
        let extension_name: String = ext.name.to_string();
        let version: String = ext.version;
        let src = format!("https://open-vsx.org/api/{publisher}/{extName}/{version}/file/{publisher}.{extName}-{version}.vsix", publisher=&publisher, extName=&extension_name, version=&version);
        let src_clone = &src.to_string();
        let readme = &ext.files.readme;

        let long_description: String = task::block_in_place(move || {
            Handle::current().block_on(async move {
                // do something async
                get_long_description(readme)
                    .await
                    .expect("Error: unable to get readme of extension")
            })
        });

        let sha256: String = task::block_in_place(move || {
            Handle::current().block_on(async move {
                // do something async
                get_hash(src_clone)
                    .await
                    .expect("Error: unable to get hash of extension's vsix")
            })
        });

        let description = if !&ext.description.is_empty() {
            Some(String::from(&ext.description))
        } else {
            None
        };

        let long_description = if !long_description.is_empty() {
            Some(long_description)
        } else {
            None
        };

        let homepage = if !&ext.publishedBy.homepage.is_empty() {
            Some(String::from(&ext.publishedBy.homepage))
        } else {
            None
        };

        let nix_license = NixLicense::from_str(&ext.license);

        let license = nix_license.map(|lic| vec![*lic]);

        let meta = NixPackageMeta {
            description,
            long_description,
            homepage,
            license,
            changelog: Some(vec![ext.files.changelog]),
            ..Default::default()
        };

        NixPackage {
            kind: PackageKind::VscodeExtension {
                publisher: publisher.to_string(),
                extension_name: extension_name.to_string(),
            },
            name: format!("{}.{}", &publisher, &extension_name),
            pname: extension_name,
            src,
            version,
            sha256,
            meta,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_openvsx() {
        let expected: OpenVSXExtensionResponse = serde_json::from_value(json!({
            "namespaceUrl":"https://open-vsx.org/api/usernamehw",
            "reviewsUrl":"https://open-vsx.org/api/usernamehw/indent-one-space/reviews",
            "files":{
                "changelog":"https://open-vsx.org/api/usernamehw/indent-one-space/0.2.6/file/CHANGELOG.md",
                "download":"https://open-vsx.org/api/usernamehw/indent-one-space/0.2.6/file/usernamehw.indent-one-space-0.2.6.vsix",
                "readme":"https://open-vsx.org/api/usernamehw/indent-one-space/0.2.6/file/README.md",
                "icon":"https://open-vsx.org/api/usernamehw/indent-one-space/0.2.6/file/icon.png",
                "manifest":"https://open-vsx.org/api/usernamehw/indent-one-space/0.2.6/file/package.json"
            },
            "name":"indent-one-space",
            "namespace":"usernamehw",
            "version":"0.2.6",
            "publishedBy":{
                "loginName":"usernamehw",
                "fullName":"Alexander",
                "avatarUrl":"https://avatars2.githubusercontent.com/u/9638156?v=4",
                "homepage":"https://github.com/usernamehw",
                "provider":"github"
            },
            "verified":true,
            "unrelatedPublisher":false,
            "namespaceAccess":"restricted",
            "allVersions":{
                "latest":"https://open-vsx.org/api/usernamehw/indent-one-space/latest",
                "0.2.6":"https://open-vsx.org/api/usernamehw/indent-one-space/0.2.6"
            },
            "downloadCount":294,
            "reviewCount":0,
            "versionAlias":[
                "latest"
            ],
            "timestamp":"2020-06-16T13:41:21.518786Z",
            "preview":false,
            "displayName":"Indent one space",
            "description":"Move code to left or right with a distance of one whitespace",
            "engines":{
                "vscode":"^1.17.0"
            },
            "categories":[
                "Other"
            ],
            "extensionKind":[],
            "tags":[
                "indentation",
                "indent"
            ],
            "license":"MIT",
            "repository":"https://github.com/usernamehw/vscode-indent-one-space",
            "bugs":"https://github.com/usernamehw/vscode-indent-one-space/issues",
            "galleryColor":"#333333",
            "galleryTheme":"dark",
            "dependencies":[],
            "bundledExtensions":[]
        })).unwrap();

        let actual: OpenVSXExtensionResponse =
            OpenVSXExtensionResponse::new("usernamehw.indent-one-space")
                .await
                .unwrap();

        assert_eq!(actual.namespaceUrl, expected.namespaceUrl);
        assert_eq!(actual.reviewsUrl, expected.reviewsUrl);
        assert_eq!(actual.files, expected.files);
        assert_eq!(actual.name, expected.name);
        assert_eq!(actual.namespace, expected.namespace);
        assert_eq!(actual.version, expected.version);
        assert_eq!(actual.publishedBy, expected.publishedBy);
        assert_eq!(actual.verified, expected.verified);
        assert_eq!(actual.unrelatedPublisher, expected.unrelatedPublisher);
        assert_eq!(actual.namespaceAccess, expected.namespaceAccess);
        assert_eq!(actual.allVersions, expected.allVersions);
        assert_eq!(actual.versionAlias, expected.versionAlias);
        assert_eq!(actual.timestamp, expected.timestamp);
        assert_eq!(actual.preview, expected.preview);
        assert_eq!(actual.displayName, expected.displayName);
        assert_eq!(actual.description, expected.description);
        assert_eq!(actual.engines, expected.engines);
        assert_eq!(actual.categories, expected.categories);
        assert_eq!(actual.extensionKind, expected.extensionKind);
        assert_eq!(actual.tags, expected.tags);
        assert_eq!(actual.license, expected.license);
        assert_eq!(actual.repository, expected.repository);
        assert_eq!(actual.bugs, expected.bugs);
        assert_eq!(actual.galleryColor, expected.galleryColor);
        assert_eq!(actual.dependencies, expected.dependencies);
        assert_eq!(actual.bundledExtensions, expected.bundledExtensions);
    }
}
