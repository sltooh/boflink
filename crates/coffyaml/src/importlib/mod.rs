use archconfig::ArchitectureConfig;
use serde::{Deserialize, Serialize};

pub use object::Architecture;

mod archconfig;
mod build;
pub mod errors;
mod legacy_build;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ImportlibYaml {
    pub library: String,
    pub exports: Vec<String>,
}
