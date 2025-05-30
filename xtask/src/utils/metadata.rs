use std::sync::LazyLock;

use serde::Deserialize;

use super::{env::exe::cargo, shell::check_output_projdir};

static CARGO_METADATA: LazyLock<Result<CargoMetadata, std::io::Error>> = LazyLock::new(|| {
    let output = check_output_projdir(cargo(), ["metadata", "--no-deps", "--format-version=1"])?;
    serde_json::from_str::<CargoMetadata>(&output).map_err(std::io::Error::other)
});

#[derive(Debug, Deserialize)]
struct CargoMetadata {
    packages: Vec<CargoMetadataPackage>,
    target_directory: String,
    workspace_root: String,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct CargoMetadataPackage {
    name: String,
    version: String,
    id: String,
    license: Option<String>,
    license_file: Option<String>,
    description: Option<String>,
    source: Option<String>,
    dependencies: Vec<CargoMetadataPackageDependency>,
    targets: Vec<CargoMetadataPackageTarget>,
    manifest_path: String,
    authors: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct CargoMetadataPackageDependency {
    name: String,
    source: Option<String>,
    req: String,
    kind: Option<String>,
    rename: Option<String>,
    optional: bool,
    uses_default_features: bool,
    features: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct CargoMetadataPackageTarget {
    kind: Vec<String>,
    crate_types: Vec<String>,
    name: String,
    src_path: String,
    edition: String,
    doc: bool,
    doctest: bool,
    test: bool,
}

pub fn workspace_root() -> std::io::Result<String> {
    CARGO_METADATA
        .as_ref()
        .map_err(std::io::Error::other)
        .map(|meta| meta.workspace_root.clone())
}

pub fn target_directory() -> std::io::Result<String> {
    CARGO_METADATA
        .as_ref()
        .map_err(std::io::Error::other)
        .map(|meta| meta.target_directory.clone())
}

pub fn package_version(name: impl AsRef<str>) -> std::io::Result<String> {
    CARGO_METADATA
        .as_ref()
        .map_err(std::io::Error::other)
        .and_then(|meta| {
            meta.packages
                .iter()
                .find_map(|package| {
                    (package.name == name.as_ref()).then(|| package.version.clone())
                })
                .ok_or_else(|| std::io::Error::other("could not find package in metadata"))
        })
}
