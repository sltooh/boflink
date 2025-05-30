use crate::archive::builder::ArchiveBuilder;

use super::{Architecture, ArchitectureConfig, ImportlibYaml, errors::ImportlibYamlBuildError};

impl ImportlibYaml {
    pub fn build_legacy(self, arch: Architecture) -> Result<Vec<u8>, ImportlibYamlBuildError> {
        let _cfg = ArchitectureConfig::new(arch)?;

        let _archive_builder = ArchiveBuilder::gnu_archive_with_capacity(0);
        todo!()
    }
}
