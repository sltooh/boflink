use std::path::Path;

use object::Architecture;

use crate::{
    libsearch::LibraryFind,
    linker::{ApiInit, ApiInitCtx, error::ApiInitError},
    linkobject::import::{ImportMember, ImportName, ImportType},
};

use super::{ApiSymbolError, ApiSymbolSource};

/// The Beacon API symbol string values.
///
/// Symbols are sorted based on commonality.
const BEACONAPI_SYMBOLS: [&str; 52] = [
    "BeaconPrintf",
    "BeaconDataParse",
    "BeaconOutput",
    "BeaconDataExtract",
    "BeaconDataInt",
    "BeaconGetSpawnTo",
    "BeaconCleanupProcess",
    "BeaconSpawnTemporaryProcess",
    "BeaconDataShort",
    "toWideChar",
    "BeaconUseToken",
    "BeaconGetValue",
    "BeaconRemoveValue",
    "BeaconInjectProcess",
    "BeaconDataLength",
    "BeaconAddValue",
    "BeaconRevertToken",
    "BeaconOpenThread",
    "BeaconUnmapViewOfFile",
    "BeaconFormatInt",
    "BeaconGetSyscallInformation",
    "BeaconDataStoreProtectItem",
    "BeaconFormatFree",
    "BeaconDataStoreUnprotectItem",
    "BeaconInformation",
    "BeaconDataStoreMaxEntries",
    "BeaconDuplicateHandle",
    "BeaconOpenProcess",
    "BeaconDataStoreGetItem",
    "BeaconEnableBeaconGate:",
    "BeaconVirtualQuery",
    "BeaconWriteProcessMemory",
    "BeaconSetThreadContext",
    "BeaconVirtualProtect",
    "BeaconFormatAppend",
    "BeaconDisableBeaconGate",
    "BeaconResumeThread",
    "BeaconDataPtr",
    "BeaconGetThreadContext",
    "BeaconIsAdmin",
    "BeaconVirtualAlloc",
    "BeaconCloseHandle",
    "BeaconReadProcessMemory",
    "BeaconFormatReset",
    "BeaconVirtualAllocEx",
    "BeaconFormatPrintf",
    "BeaconFormatToString",
    "BeaconInjectTemporaryProcess",
    "BeaconVirtualFree",
    "BeaconGetCustomUserData",
    "BeaconVirtualProtectEx",
    "BeaconFormatAlloc",
];

/// Container for looking up Beacon API symbols.
pub struct BeaconApiSymbols {
    architecture: Architecture,
    symbols: [&'static str; 52],
}

impl BeaconApiSymbols {
    /// Returns the Beacon API symbols for the specified architecture.
    pub fn new(arch: Architecture) -> BeaconApiSymbols {
        Self {
            architecture: arch,
            symbols: BEACONAPI_SYMBOLS,
        }
    }
}

impl<'a> ApiSymbolSource<'a> for BeaconApiSymbols {
    fn api_path(&self) -> &std::path::Path {
        Path::new("Beacon API")
    }

    fn extract_api_symbol(&self, symbol: &'a str) -> Result<ImportMember<'a>, ApiSymbolError> {
        let unprefixed_symbol = if self.architecture == Architecture::I386 {
            symbol.trim_start_matches("__imp__")
        } else {
            symbol.trim_start_matches("__imp_")
        };

        self.symbols
            .iter()
            .copied()
            .find_map(|contained_symbol| {
                (contained_symbol == unprefixed_symbol).then_some(ImportMember {
                    architecture: self.architecture,
                    symbol: contained_symbol,
                    dll: "Beacon API",
                    import: ImportName::Name(contained_symbol),
                    typ: ImportType::Code,
                })
            })
            .ok_or(ApiSymbolError::NotFound)
    }
}

pub struct BeaconApiInit;

impl ApiInit for BeaconApiInit {
    type Output<'a> = BeaconApiSymbols;

    #[inline]
    fn initialize_api<'a, L: LibraryFind>(
        &self,
        ctx: &ApiInitCtx<'_, 'a, L>,
    ) -> Result<Self::Output<'a>, ApiInitError> {
        Ok(BeaconApiSymbols::new(ctx.target_arch.into()))
    }
}
