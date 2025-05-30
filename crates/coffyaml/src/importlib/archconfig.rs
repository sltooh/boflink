use object::pe::{IMAGE_FILE_MACHINE_AMD64, IMAGE_REL_AMD64_ADDR32NB};

use super::{Architecture, errors::ImportlibYamlBuildError};

pub(super) struct ArchitectureConfig {
    machine: u16,
    reloc_type: u16,
}

impl ArchitectureConfig {
    pub fn new(arch: Architecture) -> Result<ArchitectureConfig, ImportlibYamlBuildError> {
        Ok(match arch {
            Architecture::X86_64 => Self {
                machine: IMAGE_FILE_MACHINE_AMD64,
                reloc_type: IMAGE_REL_AMD64_ADDR32NB,
            },
            _ => return Err(ImportlibYamlBuildError::UnsupportArchitecture(arch)),
        })
    }

    #[inline]
    pub fn machine(&self) -> u16 {
        self.machine
    }

    #[inline]
    pub fn reloc_type(&self) -> u16 {
        self.reloc_type
    }
}
