use std::path::{Path, PathBuf};

use num_enum::{IntoPrimitive, TryFromPrimitive};
use object::pe::{IMAGE_FILE_MACHINE_AMD64, IMAGE_FILE_MACHINE_I386};
use typed_arena::Arena;

use crate::{
    api::ApiSymbolSource, libsearch::LibraryFind, linkobject::archive::LinkArchive,
    pathed_item::PathedItem,
};
use error::{ApiInitError, LinkError};

mod builder;
mod configured;
pub mod error;

pub use self::configured::*;
pub use builder::*;

pub trait LinkImpl {
    fn link(&mut self) -> Result<Vec<u8>, LinkError>;
}

#[derive(Clone, Copy, PartialEq, Eq, TryFromPrimitive, IntoPrimitive)]
#[repr(u16)]
pub enum LinkerTargetArch {
    Amd64 = IMAGE_FILE_MACHINE_AMD64,
    I386 = IMAGE_FILE_MACHINE_I386,
}

impl From<LinkerTargetArch> for object::Architecture {
    fn from(value: LinkerTargetArch) -> Self {
        match value {
            LinkerTargetArch::Amd64 => object::Architecture::X86_64,
            LinkerTargetArch::I386 => object::Architecture::I386,
        }
    }
}

impl TryFrom<object::Architecture> for LinkerTargetArch {
    type Error = object::Architecture;

    fn try_from(value: object::Architecture) -> Result<Self, Self::Error> {
        Ok(match value {
            object::Architecture::X86_64 => Self::Amd64,
            object::Architecture::I386 => Self::I386,
            _ => return Err(value),
        })
    }
}

pub struct ApiInitCtx<'b, 'a, L: LibraryFind> {
    pub(super) target_arch: LinkerTargetArch,
    pub(super) library_searcher: &'b L,
    pub(super) arena: &'a Arena<PathedItem<PathBuf, Vec<u8>>>,
}

pub trait ApiInit {
    type Output<'a>: ApiSymbolSource<'a>;

    fn initialize_api<'a, L: LibraryFind>(
        &self,
        ctx: &ApiInitCtx<'_, 'a, L>,
    ) -> Result<Self::Output<'a>, ApiInitError>;
}

struct CustomApiInit(String);

impl From<String> for CustomApiInit {
    fn from(v: String) -> CustomApiInit {
        Self(v)
    }
}

impl ApiInit for CustomApiInit {
    type Output<'a> = PathedItem<&'a Path, LinkArchive<'a>>;

    fn initialize_api<'a, L: LibraryFind>(
        &self,
        ctx: &ApiInitCtx<'_, 'a, L>,
    ) -> Result<Self::Output<'a>, ApiInitError> {
        let custom_api = match std::fs::read(&self.0) {
            Ok(buffer) => ctx
                .arena
                .alloc(PathedItem::new(PathBuf::from(&self.0), buffer)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                match ctx.library_searcher.find_library(&self.0) {
                    Ok(found) => ctx.arena.alloc(found),
                    Err(e) => {
                        return Err(e.into());
                    }
                }
            }
            Err(e) => {
                return Err(ApiInitError::Io {
                    path: PathBuf::from(&self.0),
                    error: e,
                });
            }
        };

        let parsed =
            LinkArchive::parse(custom_api.as_slice()).map_err(|e| ApiInitError::Parse {
                path: custom_api.path().to_path_buf(),
                error: e,
            })?;

        Ok(PathedItem::new(custom_api.path().as_path(), parsed))
    }
}
