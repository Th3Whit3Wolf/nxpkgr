use serde::de;
use serde::{Deserialize, Serialize};

use color_eyre::{
    eyre::{eyre, Report, Result, WrapErr},
    Section,
};

use reqwest::header::{
    HeaderMap, HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE, REFERER, USER_AGENT,
};
use tokio::{runtime::Handle, task};

use crate::{
    nix::{NixLicense, NixPackage, NixPackageMeta},
    sources::{get_hash, get_long_description},
};

const EXT_QUERY_ADDRESS: &str =
    "https://marketplace.visualstudio.com/_apis/public/gallery/extensionquery";

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payload {
    pub filters: Vec<PayloadCriteria>,
    pub asset_types: Vec<String>,
    pub flags: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayloadCriteria {
    pub criteria: Vec<PayloadCriterion>,
    pub direction: u64,
    pub page_number: u64,
    pub page_size: u64,
    pub sort_by: u64,
    pub sort_order: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayloadCriterion {
    pub filter_type: u64,
    pub value: String,
}

impl Payload {
    pub fn new(unique_id: String) -> Self {
        Payload {
            filters: vec![PayloadCriteria {
                criteria: vec![PayloadCriterion {
                    filter_type: 7,
                    value: unique_id,
                }],
                direction: 2,
                page_number: 1,
                page_size: 100,
                sort_by: 0,
                sort_order: 0,
            }],
            asset_types: Vec::with_capacity(1),
            flags: 103,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VSMarketPlaceQueryResultResponse {
    pub results: Vec<VSMarketPlaceQueryResults>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VSMarketPlaceQueryResults {
    pub extensions: Vec<VSMarketPlaceExtension>,
    pub paging_token: Option<String>,
    pub result_metadata: Vec<VSMarketPlaceQueryResultMetaData>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VSMarketPlaceExtension {
    pub publisher: VSMarketPlaceExtensionPublisher,
    pub extension_id: String,
    pub extension_name: String,
    pub display_name: String,
    pub flags: String,
    pub last_updated: String,
    pub published_date: String,
    pub release_date: String,
    pub short_description: Option<String>,
    pub versions: Vec<VSMarketPlaceExtensionVersion>,
    pub categories: Vec<String>,
    pub tags: Vec<String>,
    pub installation_targets: Vec<VSMarketPlaceExtensionInstallationTarget>,
    pub deployment_type: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VSMarketPlaceExtensionPublisher {
    pub publisher_id: String,
    pub publisher_name: String,
    pub display_name: String,
    pub flags: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VSMarketPlaceExtensionVersion {
    pub version: String,
    pub flags: String,
    pub last_updated: String,
    pub files: Vec<VSMarketPlaceExtensionVersionFile>,
    pub asset_uri: String,
    pub fallback_asset_uri: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VSMarketPlaceExtensionVersionFile {
    pub asset_type: AssetTypeMicrosoftVisualStudio,
    pub source: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum AssetTypeMicrosoftVisualStudio {
    #[serde(rename = "Microsoft.VisualStudio.Code.Manifest")]
    CodeManifest,
    #[serde(rename = "Microsoft.VisualStudio.Services.Content.Changelog")]
    ServicesContentChangelog,
    #[serde(rename = "Microsoft.VisualStudio.Services.Content.Details")]
    ServicesContentDetails,
    #[serde(rename = "Microsoft.VisualStudio.Services.Content.License")]
    ServicesContentLicense,
    #[serde(rename = "Microsoft.VisualStudio.Services.Icons.Default")]
    ServicesIconsDefault,
    #[serde(rename = "Microsoft.VisualStudio.Services.Icons.Small")]
    ServicesIconsSmall,
    #[serde(rename = "Microsoft.VisualStudio.Services.VsixManifest")]
    ServicesVSIXManifest,
    #[serde(rename = "Microsoft.VisualStudio.Services.VSIXPackage")]
    ServicesVSIXPackage,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VSMarketPlaceExtensionInstallationTarget {
    pub target: String,
    pub target_version: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VSMarketPlaceQueryResultMetaData {
    pub metadata_type: String,
    pub metadata_items: Vec<VSMarketPlaceQueryResultMetaDataItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct VSMarketPlaceQueryResultMetaDataItem {
    pub name: String,
    pub count: u64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VSMarketPlaceExtensionRefined {
    pub publisher: String,
    pub extension_name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub version: String,
    pub vsix_url: String,
    pub vsix_manifest_url: String,
    pub changelog_url: Option<String>,
    pub readme_url: Option<String>,
}

impl VSMarketPlaceExtensionRefined {
    pub async fn get(unique_id: String) -> Result<Self, Report> {
        let data = Payload::new(unique_id);

        let mut headers = HeaderMap::new();

        // Declare headers
        headers.insert(
            ACCEPT,
            HeaderValue::from_static("application/json;api-version=6.1-preview.1"),
        );
        headers.insert(REFERER, HeaderValue::from_static(""));
        headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_5) AppleWebKit/537.36 (KHTML, like Gecko) Code/1.51.0 Chrome/83.0.4103.122 Electron/9.3.3 Safari/537.36"));
        headers.insert(
            HeaderName::from_static("x-market-client-id"),
            HeaderValue::from_static("VSCode 1.51.0"),
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let response = reqwest::Client::new()
            .post(EXT_QUERY_ADDRESS)
            .headers(headers)
            .json(&data)
            .send()
            .await?;

        let resp_status = response.status();

        if resp_status.is_success() {
            let mut vsmarketplace_response_json: VSMarketPlaceQueryResultResponse =
                match response.json().await {
                    Ok(query_result) => query_result,
                    Err(e) => {
                        return Err(eyre!("Unable to parse json from vscode marketplace")).error(e)
                    }
                };
            if !vsmarketplace_response_json.results.is_empty() {
                let mut query_result = vsmarketplace_response_json.results.remove(0);
                vsmarketplace_response_json.results.clear();

                if !query_result.extensions.is_empty() {
                    let mut extension = query_result.extensions.remove(0);
                    query_result.extensions.clear();

                    let mut vsix_url = Box::new(String::new());
                    let mut vsix_manifest_url = Box::new(String::new());
                    let mut changelog_url_box = Box::new(String::new());
                    let mut readme_url_box = Box::new(String::new());

                    let version_struct = extension.versions.remove(0);
                    extension.versions.clear();

                    for file in version_struct.files {
                        match file.asset_type {
                            AssetTypeMicrosoftVisualStudio::ServicesVSIXPackage => {
                                *vsix_url = file.source;
                            }
                            AssetTypeMicrosoftVisualStudio::ServicesVSIXManifest => {
                                *vsix_manifest_url = file.source;
                            }
                            AssetTypeMicrosoftVisualStudio::ServicesContentChangelog => {
                                *changelog_url_box = file.source;
                            }
                            AssetTypeMicrosoftVisualStudio::ServicesContentDetails => {
                                *readme_url_box = file.source;
                            }
                            _ => (),
                        }
                    }

                    let changelog_url: Option<String> = if *changelog_url_box != String::new() {
                        Some(*changelog_url_box)
                    } else {
                        None
                    };

                    let readme_url: Option<String> = if *readme_url_box != String::new() {
                        Some(*readme_url_box)
                    } else {
                        None
                    };

                    if *vsix_url != String::new() && *vsix_manifest_url != String::new() {
                        Ok(VSMarketPlaceExtensionRefined {
                            publisher: extension.publisher.publisher_name,
                            extension_name: extension.extension_name,
                            display_name: extension.display_name,
                            description: extension.short_description,
                            version: version_struct.version,
                            vsix_url: *vsix_url,
                            vsix_manifest_url: *vsix_manifest_url,
                            changelog_url,
                            readme_url,
                        })
                    } else {
                        return Err(eyre!("No VSIX or VSIX Manifest found"));
                    }
                } else {
                    return Err(eyre!("No extensions found from vscode marketplace query"));
                }
            } else {
                return Err(eyre!("No results found from vscode marketplace"));
            }
        } else if let Some(reason) = resp_status.canonical_reason() {
            return Err(eyre!(
                "Recieved {}, while attempting to get extension from vscode marketplace.",
                reason
            ));
        } else {
            return Err(eyre!("{}", resp_status.to_string()));
        }
    }

    pub async fn get_with_version(unique_id: String, version: String) -> Result<Self, Report> {
        let data = Payload::new(unique_id);

        let mut headers = HeaderMap::new();

        // Declare headers
        headers.insert(
            ACCEPT,
            HeaderValue::from_static("application/json;api-version=6.1-preview.1"),
        );
        headers.insert(REFERER, HeaderValue::from_static(""));
        headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_5) AppleWebKit/537.36 (KHTML, like Gecko) Code/1.51.0 Chrome/83.0.4103.122 Electron/9.3.3 Safari/537.36"));
        headers.insert(
            HeaderName::from_static("x-market-client-id"),
            HeaderValue::from_static("VSCode 1.51.0"),
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let response = reqwest::Client::new()
            .post(EXT_QUERY_ADDRESS)
            .headers(headers)
            .json(&data)
            .send()
            .await?;

        let resp_status = response.status();

        if resp_status.is_success() {
            let mut vsmarketplace_response_json: VSMarketPlaceQueryResultResponse =
                match response.json().await {
                    Ok(query_result) => query_result,
                    Err(e) => {
                        return Err(eyre!("Unable to parse json from vscode marketplace")).error(e)
                    }
                };
            if !vsmarketplace_response_json.results.is_empty() {
                let mut query_result = vsmarketplace_response_json.results.remove(0);
                vsmarketplace_response_json.results.clear();

                if !query_result.extensions.is_empty() {
                    let extension = query_result.extensions.remove(0);
                    query_result.extensions.clear();

                    let mut vsix_url = Box::new(String::new());
                    let mut vsix_manifest_url = Box::new(String::new());
                    let mut changelog_url_box = Box::new(String::new());
                    let mut readme_url_box = Box::new(String::new());

                    let mut vers: Option<VSMarketPlaceExtensionVersion> = None;

                    for v in extension.versions {
                        if v.version == version {
                            vers = Some(v);
                        }
                    }

                    if let Some(v) = vers {
                        for file in v.files {
                            match file.asset_type {
                                AssetTypeMicrosoftVisualStudio::ServicesVSIXPackage => {
                                    *vsix_url = file.source;
                                }
                                AssetTypeMicrosoftVisualStudio::ServicesVSIXManifest => {
                                    *vsix_manifest_url = file.source;
                                }
                                AssetTypeMicrosoftVisualStudio::ServicesContentChangelog => {
                                    *changelog_url_box = file.source;
                                }
                                AssetTypeMicrosoftVisualStudio::ServicesContentDetails => {
                                    *readme_url_box = file.source;
                                }
                                _ => (),
                            }
                        }

                        let changelog_url: Option<String> = if *changelog_url_box != String::new() {
                            Some(*changelog_url_box)
                        } else {
                            None
                        };

                        let readme_url: Option<String> = if *readme_url_box != String::new() {
                            Some(*readme_url_box)
                        } else {
                            None
                        };

                        if *vsix_url != String::new() && *vsix_manifest_url != String::new() {
                            Ok(VSMarketPlaceExtensionRefined {
                                publisher: extension.publisher.publisher_name,
                                extension_name: extension.extension_name,
                                display_name: extension.display_name,
                                description: extension.short_description,
                                version: v.version,
                                vsix_url: *vsix_url,
                                vsix_manifest_url: *vsix_manifest_url,
                                changelog_url,
                                readme_url,
                            })
                        } else {
                            return Err(eyre!("No VSIX or VSIX Manifest found"));
                        }
                    } else {
                        return Err(eyre!(
                            "No version found for extensions from vscode marketplace query"
                        ));
                    }
                } else {
                    return Err(eyre!("No extensions found from vscode marketplace query"));
                }
            } else {
                return Err(eyre!("No results found from vscode marketplace"));
            }
        } else if let Some(reason) = resp_status.canonical_reason() {
            return Err(eyre!(
                "Recieved {}, while attempting to get extension from vscode marketplace.",
                reason
            ));
        } else {
            return Err(eyre!("{}", resp_status.to_string()));
        }
    }
    pub fn to_nixpkg(self, pname: String) -> NixPackage {
        let publisher: String = self.publisher.clone();
        let extension_name: String = self.extension_name.clone();
        let version: String = self.version.clone();
        let src = format!("https://{publisher}.gallery.vsassets.io/_apis/public/gallery/publisher/{publisher}/extension/{extName}/{version}/assetbyname/Microsoft.VisualStudio.Services.VSIXPackage", publisher=&publisher, extName=&extension_name, version=&version);
        let src_clone = &src.to_string();
        let description = self.description.clone();
        let changelog = self.changelog_url.clone().map(|change| vec![change]);

        let sha256: String = task::block_in_place(move || {
            Handle::current().block_on(async move {
                get_hash(src_clone)
                    .await
                    .expect("Error: unable to get hash of vsix")
            })
        });

        let (homepage, github, source) = task::block_in_place(move || {
            Handle::current().block_on(async move {
                let mut homepage_box = Box::new(String::from(""));
                let mut github_box = Box::new(String::from(""));
                let mut source_box = Box::new(String::from(""));

                if let Ok(doc) = roxmltree::Document::parse(self.vsix_manifest_url.as_ref()) {
                    for node in doc.descendants() {
                        if node.is_element() && node.has_tag_name("Property") {
                            if let Some(property_id) = node.attribute_node("Id") {
                                match property_id.value() {
                                    "Microsoft.VisualStudio.Services.Links.Learn" => {
                                        if let Some(property_value) = node.attribute_node("Value") {
                                            *homepage_box = property_value.value().to_string()
                                        }
                                    }
                                    "Microsoft.VisualStudio.Services.Links.GitHub" => {
                                        if let Some(property_value) = node.attribute_node("Value") {
                                            *github_box = property_value.value().to_string()
                                        }
                                    }
                                    "Microsoft.VisualStudio.Services.Links.Source" => {
                                        if let Some(property_value) = node.attribute_node("Value") {
                                            *source_box = property_value.value().to_string()
                                        }
                                    }
                                    _ => (),
                                }
                            }
                        }
                    }

                    let homepage: Option<String> = if *homepage_box != String::new() {
                        Some(*homepage_box)
                    } else {
                        None
                    };

                    let github: Option<String> = if *github_box != String::new() {
                        Some(*github_box)
                    } else {
                        None
                    };

                    let source: Option<String> = if *source_box != String::new() {
                        Some(*source_box)
                    } else {
                        None
                    };
                    (homepage, github, source)
                } else {
                    (None, None, None)
                }
            })
        });

        let long_description: Option<String> = if let Some(s) = source {
            let desc = task::block_in_place(move || {
                Handle::current().block_on(async move {
                    // do something async
                    get_long_description(s)
                        .await
                        .expect("Error: unable to get readme of extension")
                })
            });

            if desc == String::new() {
                None
            } else {
                Some(desc)
            }
        } else {
            None
        };

        let license = if let Some(github_url) = github {
            task::block_in_place(move || {
                Handle::current().block_on(async move {
                    // do something async
                    let github = github_url.trim_end_matches(".git");
                    let github = github.trim_start_matches("https://github.com/");
                    let split_github_url: Vec<&str> = github.split('/').collect();
                    let github_author = split_github_url[0];
                    let github_repo = split_github_url[1];
                    if let Some(lic) = octocrab::instance()
                        .repos(github_author, github_repo)
                        .license()
                        .await
                        .expect("Error: getting repo information")
                        .license
                    {
                        NixLicense::from_str(&lic.name)
                    } else {
                        None
                    }
                })
            })
            .map(|lic| vec![*lic])
        } else {
            None
        };

        let meta = NixPackageMeta {
            description,
            long_description,
            homepage,
            license,
            changelog,
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

    /*
        pub struct VSMarketPlaceExtensionRefined {
        pub publisher: String,
        pub extension_name: String,
        pub display_name: String,
        pub description: Option<String>,
        pub version: String,
        pub vsix_url: String,
        pub vsix_manifest_url: String,
        pub changelog_url: Option<String>,
        pub readme_url: Option<String>
    }

    */
    #[tokio::test]
    async fn test_get() {
        let expected: VSMarketPlaceExtensionRefined = serde_json::from_value(json!({
            "publisher": "cometeer",
            "extensionName": "spacemacs",
            "displayName": "Spacemacs",
            "description": "Spacemacs themes for Visual Studio Code",
            "version": "1.1.1",
            "vsixUrl": "https://cometeer.gallerycdn.vsassets.io/extensions/cometeer/spacemacs/1.1.1/1507198251877/Microsoft.VisualStudio.Services.VSIXPackage",
            "vsixManifestUrl": "https://cometeer.gallerycdn.vsassets.io/extensions/cometeer/spacemacs/1.1.1/1507198251877/Microsoft.VisualStudio.Services.VsixManifest",
            "changelogUrl": "https://cometeer.gallerycdn.vsassets.io/extensions/cometeer/spacemacs/1.1.1/1507198251877/Microsoft.VisualStudio.Services.Content.Changelog",
            "readmeUrl": "https://cometeer.gallerycdn.vsassets.io/extensions/cometeer/spacemacs/1.1.1/1507198251877/Microsoft.VisualStudio.Services.Content.Details"
	    })).unwrap();

        let actual: VSMarketPlaceExtensionRefined =
            VSMarketPlaceExtensionRefined::get(String::from("cometeer.spacemacs"))
                .await
                .unwrap();
        assert_eq!(actual.publisher, expected.publisher);
        assert_eq!(actual.extension_name, expected.extension_name);
        assert_eq!(actual.display_name, expected.display_name);
        assert_eq!(actual.description, expected.description);
        assert_eq!(actual.version, expected.version);
        assert_eq!(actual.vsix_url, expected.vsix_url);
        assert_eq!(actual.vsix_manifest_url, expected.vsix_manifest_url);
        assert_eq!(actual.changelog_url, expected.changelog_url);
        assert_eq!(actual.readme_url, expected.readme_url);
    }

    #[tokio::test]
    async fn test_get_with_version() {
        let expected: VSMarketPlaceExtensionRefined = serde_json::from_value(json!({
            "publisher": "cometeer",
            "extensionName": "spacemacs",
            "displayName": "Spacemacs",
            "description": "Spacemacs themes for Visual Studio Code",
            "version": "1.1.0",
            "vsixUrl": "https://cometeer.gallerycdn.vsassets.io/extensions/cometeer/spacemacs/1.1.0/1507198207264/Microsoft.VisualStudio.Services.VSIXPackage",
            "vsixManifestUrl": "https://cometeer.gallerycdn.vsassets.io/extensions/cometeer/spacemacs/1.1.0/1507198207264/Microsoft.VisualStudio.Services.VsixManifest",
            "changelogUrl": "https://cometeer.gallerycdn.vsassets.io/extensions/cometeer/spacemacs/1.1.0/1507198207264/Microsoft.VisualStudio.Services.Content.Changelog",
            "readmeUrl": "https://cometeer.gallerycdn.vsassets.io/extensions/cometeer/spacemacs/1.1.0/1507198207264/Microsoft.VisualStudio.Services.Content.Details"
	    })).unwrap();

        let actual: VSMarketPlaceExtensionRefined =
            VSMarketPlaceExtensionRefined::get_with_version(
                String::from("cometeer.spacemacs"),
                String::from("1.1.0"),
            )
            .await
            .unwrap();
        assert_eq!(actual.publisher, expected.publisher);
        assert_eq!(actual.extension_name, expected.extension_name);
        assert_eq!(actual.display_name, expected.display_name);
        assert_eq!(actual.description, expected.description);
        assert_eq!(actual.version, expected.version);
        assert_eq!(actual.vsix_url, expected.vsix_url);
        assert_eq!(actual.vsix_manifest_url, expected.vsix_manifest_url);
        assert_eq!(actual.changelog_url, expected.changelog_url);
        assert_eq!(actual.readme_url, expected.readme_url);
    }
}
