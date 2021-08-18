use serde::{Deserialize, Serialize};

use color_eyre::{
    eyre::{eyre, Report, WrapErr, Result},
    Section,
};

use std::collections::HashMap;

use crate::{
    nix::{NixLicense, NixPackage, NixPackageMeta},
    sources::{get_hash, get_long_description},
};

use tokio::{runtime::Handle, task};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenVSXExtension {
    pub namespace_url: String,
    pub reviews_url: String,
    pub files: OpenVSXExtensionFile,
    pub name: String,
    pub namespace: String,
    pub version: String,
    pub published_by: OpenVSXExtensionPublishedBy,
    pub verified: bool,
    pub unrelated_publisher: bool,
    pub namespace_access: String,
    pub all_versions: HashMap<String, String>,
    pub average_rating: Option<f64>,
    pub download_count: Option<u64>,
    pub review_count: u64,
    pub version_alias: Vec<String>,
    pub timestamp: String,
    pub preview: bool,
    pub display_name: String,
    pub description: String,
    pub engines: HashMap<String, String>,
    pub categories: Vec<String>,
    pub extension_kind: Vec<String>,
    pub tags: Vec<String>,
    pub license: String,
    pub repository: String,
    pub bugs: String,
    pub gallery_color: Option<String>,
    pub gallery_theme: Option<String>,
    pub dependencies: Vec<String>,
    pub bundled_extensions: Vec<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct OpenVSXExtensionFile {
    changelog: String,
    download: String,
    readme: String,
    icon: String,
    manifest: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct OpenVSXExtensionPublishedBy {
    login_name: String,
    full_name: String,
    avatar_url: String,
    homepage: String,
    provider: String,
}

impl OpenVSXExtension {
    pub async fn get(unique_id: String) -> Result<Self> {
        let split_id: Vec<&str> = unique_id.split('.').collect();
        let namespace = split_id[0];
        let extension = split_id[1];
        let url = format!(
            "https://open-vsx.org/api/{namespace}/{extension}",
            namespace = namespace,
            extension = extension
        );

        let response: OpenVSXExtension =
            reqwest::Client::new().get(url).send().await?.json().await?;

        Ok(response)
    }

    pub async fn get_with_version(unique_id: String, version: String) -> Result<Self> {
        let split_id: Vec<&str> = unique_id.split('.').collect();
        let namespace = split_id[0];
        let extension = split_id[1];
        let url = format!(
            "https://open-vsx.org/api/{namespace}/{extension}/{version}",
            namespace = namespace,
            extension = extension,
            version = version
        );

        let response: OpenVSXExtension =
            reqwest::Client::new().get(url).send().await?.json().await?;

        Ok(response)
    }

    pub fn to_nixpkg(self, pname: String, extract: Option<String>) -> NixPackage {
        let namespace: String = self.namespace;
        let extension: String = self.name;
        let version: String = self.version;
        let src = format!("https://open-vsx.org/api/{namespace}/{extension}/{version}/file/{namespace}.{extension}-{version}.vsix", namespace=&namespace, extension=&extension, version=&version);
        let src_clone = &src.to_string();
        let readme = self.files.readme;

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

        let description = if !&self.description.is_empty() {
            Some(String::from(&self.description))
        } else {
            None
        };

        let long_description = if !long_description.is_empty() {
            Some(long_description)
        } else {
            None
        };

        let homepage = if !&self.published_by.homepage.is_empty() {
            Some(String::from(&self.published_by.homepage))
        } else {
            None
        };

        let nix_license = NixLicense::from_str(&self.license);

        let license = nix_license.map(|lic| vec![*lic]);

        let meta = NixPackageMeta {
            description,
            long_description,
            homepage,
            license,
            changelog: Some(vec![self.files.changelog]),
            ..Default::default()
        };

        NixPackage {
            name: pname.clone(),
            pname,
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
    async fn test_openvsx_get() {
        let indent_one_space_0_2_6: OpenVSXExtension = serde_json::from_value(json!({
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

        let actual: OpenVSXExtension =
            OpenVSXExtension::get(String::from("usernamehw.indent-one-space"))
                .await
                .unwrap();

        assert_eq!(actual.namespace_url, indent_one_space_0_2_6.namespace_url);
        assert_eq!(actual.reviews_url, indent_one_space_0_2_6.reviews_url);
        assert_eq!(actual.name, indent_one_space_0_2_6.name);
        assert_eq!(actual.namespace, indent_one_space_0_2_6.namespace);
        assert_eq!(actual.published_by, indent_one_space_0_2_6.published_by);
        assert_eq!(actual.verified, indent_one_space_0_2_6.verified);
        assert_eq!(
            actual.unrelated_publisher,
            indent_one_space_0_2_6.unrelated_publisher
        );
        assert_eq!(
            actual.namespace_access,
            indent_one_space_0_2_6.namespace_access
        );
        assert_eq!(actual.preview, indent_one_space_0_2_6.preview);
        assert_eq!(actual.display_name, indent_one_space_0_2_6.display_name);
        assert_eq!(actual.description, indent_one_space_0_2_6.description);
        assert_eq!(actual.categories, indent_one_space_0_2_6.categories);
        assert_eq!(actual.license, indent_one_space_0_2_6.license);
        assert_eq!(actual.repository, indent_one_space_0_2_6.repository);
        assert_eq!(actual.bugs, indent_one_space_0_2_6.bugs);
        assert_eq!(actual.dependencies, indent_one_space_0_2_6.dependencies);
        assert_eq!(
            actual.bundled_extensions,
            indent_one_space_0_2_6.bundled_extensions
        );
    }

    #[tokio::test]
    async fn test_openvsx_get_with_version() {
        let indent_one_space_0_2_6: OpenVSXExtension = serde_json::from_value(json!({
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

        let actual: OpenVSXExtension = OpenVSXExtension::get_with_version(
            String::from("usernamehw.indent-one-space"),
            String::from("0.2.6"),
        )
        .await
        .unwrap();

        assert_eq!(actual.namespace_url, indent_one_space_0_2_6.namespace_url);
        assert_eq!(actual.reviews_url, indent_one_space_0_2_6.reviews_url);
        assert_eq!(actual.name, indent_one_space_0_2_6.name);
        assert_eq!(actual.version, indent_one_space_0_2_6.version);
        assert_eq!(actual.namespace, indent_one_space_0_2_6.namespace);
        assert_eq!(actual.published_by, indent_one_space_0_2_6.published_by);
        assert_eq!(actual.verified, indent_one_space_0_2_6.verified);
        assert_eq!(
            actual.unrelated_publisher,
            indent_one_space_0_2_6.unrelated_publisher
        );
        assert_eq!(
            actual.namespace_access,
            indent_one_space_0_2_6.namespace_access
        );
        assert_eq!(actual.preview, indent_one_space_0_2_6.preview);
        assert_eq!(actual.display_name, indent_one_space_0_2_6.display_name);
        assert_eq!(actual.description, indent_one_space_0_2_6.description);
        assert_eq!(actual.categories, indent_one_space_0_2_6.categories);
        assert_eq!(actual.license, indent_one_space_0_2_6.license);
        assert_eq!(actual.repository, indent_one_space_0_2_6.repository);
        assert_eq!(actual.bugs, indent_one_space_0_2_6.bugs);
        assert_eq!(actual.dependencies, indent_one_space_0_2_6.dependencies);
        assert_eq!(
            actual.bundled_extensions,
            indent_one_space_0_2_6.bundled_extensions
        );
    }
}
