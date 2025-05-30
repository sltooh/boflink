use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    ops::Deref,
    path::{Path, PathBuf},
};

use object::{
    Object,
    coff::{CoffFile, ImportFile},
    pe::IMAGE_FILE_MACHINE_UNKNOWN,
    read::archive::{ArchiveFile, ArchiveMember, ArchiveOffset, ArchiveSymbolIterator},
};

use crate::{
    api::{ApiSymbolError, ApiSymbolSource},
    pathed_item::PathedItem,
};

use super::import::ImportMember;

pub use error::*;

use legacy_importlib::{LegacyImportHeadMember, LegacyImportSymbolMember, LegacyImportTailMember};

pub mod error;
mod legacy_importlib;

pub struct ExtractedMember<'a> {
    path: &'a Path,
    contents: ExtractedMemberContents<'a>,
}

impl<'a> ExtractedMember<'a> {
    pub fn new(
        path: &'a Path,
        contents: impl Into<ExtractedMemberContents<'a>>,
    ) -> ExtractedMember<'a> {
        Self {
            path,
            contents: contents.into(),
        }
    }

    pub fn path(&self) -> &'a Path {
        self.path
    }

    pub fn contents(&self) -> &ExtractedMemberContents<'a> {
        &self.contents
    }
}

pub enum ExtractedMemberContents<'a> {
    Coff(CoffFile<'a>),
    Import(ImportMember<'a>),
}

impl<'a> From<CoffFile<'a>> for ExtractedMemberContents<'a> {
    fn from(value: CoffFile<'a>) -> Self {
        Self::Coff(value)
    }
}

impl<'a> From<ImportMember<'a>> for ExtractedMemberContents<'a> {
    fn from(value: ImportMember<'a>) -> Self {
        Self::Import(value)
    }
}

struct CachedSymbolMap<'a> {
    cache: HashMap<&'a str, ArchiveOffset>,
    iter: Option<ArchiveSymbolIterator<'a>>,
}

impl CachedSymbolMap<'_> {
    fn find_symbol(&mut self, symbol: &str) -> Option<ArchiveOffset> {
        if let Some(found) = self.cache.get(symbol).copied() {
            return Some(found);
        }

        for archive_symbol in self.iter.iter_mut().flatten().flatten() {
            let archive_symbol_name = match std::str::from_utf8(archive_symbol.name()) {
                Ok(name) => name,
                Err(_) => continue,
            };

            self.cache
                .insert(archive_symbol_name, archive_symbol.offset());
            if archive_symbol_name == symbol {
                return Some(archive_symbol.offset());
            }
        }

        None
    }
}

/// A parsed archive file for linking.
pub struct LinkArchive<'a> {
    /// The parsed archive file
    archive_file: ArchiveFile<'a>,

    /// The cached archive symbol table.
    symbol_cache: RefCell<CachedSymbolMap<'a>>,

    /// Map of legacy import member '_head_*' symbols to the associated
    /// library names.
    legacy_imports: RefCell<BTreeMap<&'a str, &'a str>>,

    /// The archive file data.
    archive_data: &'a [u8],
}

impl<'a> LinkArchive<'a> {
    /// Parses the data.
    pub fn parse(data: &'a [u8]) -> Result<LinkArchive<'a>, LinkArchiveParseError> {
        let archive_file = ArchiveFile::parse(data)?;

        if archive_file.is_thin() {
            return Err(LinkArchiveParseError::ThinArchive);
        }

        let symbols = archive_file
            .symbols()?
            .ok_or(LinkArchiveParseError::NoSymbolMap)?;
        let symbol_count = symbols
            .size_hint()
            .1
            .unwrap_or_else(|| symbols.clone().count());

        Ok(Self {
            archive_file,
            symbol_cache: RefCell::new(CachedSymbolMap {
                cache: HashMap::with_capacity(symbol_count),
                iter: Some(symbols),
            }),
            legacy_imports: RefCell::new(BTreeMap::new()),
            archive_data: data,
        })
    }

    pub fn extract_symbol(
        &self,
        symbol: &'a str,
    ) -> Result<ExtractedMember<'a>, ExtractMemberError> {
        let extracted = self.extract_archive_member(symbol)?;
        let member_name = std::str::from_utf8(extracted.name())
            .map_err(|e| ExtractMemberError::ArchiveParse(ArchiveParseError::MemberName(e)))?;

        self.parse_member(&extracted, member_name)
            .map_err(ExtractMemberError::MemberParse)
    }

    fn parse_member(
        &self,
        member: &ArchiveMember<'a>,
        member_name: &'a str,
    ) -> Result<ExtractedMember<'a>, MemberParseError> {
        let member_data = member
            .data(self.archive_data)
            .map_err(|e| MemberParseError::new(PathBuf::from(member_name), e))?;

        let member_path = Path::new(member_name);

        if member_data
            .get(..2)
            .is_some_and(|magic| magic == IMAGE_FILE_MACHINE_UNKNOWN.to_le_bytes())
        {
            Ok(ExtractedMember {
                path: member_path,
                contents: ExtractedMemberContents::Import(
                    ImportFile::parse(member_data)
                        .map_err(|e| MemberParseError::new(member_path, e))?
                        .try_into()
                        .map_err(|e| MemberParseError::new(member_path, e))?,
                ),
            })
        } else {
            let coff = CoffFile::<&[u8]>::parse(member_data)
                .map_err(|e| MemberParseError::new(member_path, e))?;

            match self.parse_legacy_import_member(member_name, &coff) {
                Ok(import) => Ok(ExtractedMember::new(member_path, import)),
                Err(e)
                    if matches!(
                        e.kind,
                        MemberParseErrorKind::LegacyImportLibrarySymbolMember(
                            LegacyImportSymbolMemberParseError::Invalid
                        )
                    ) =>
                {
                    Ok(ExtractedMember::new(member_path, coff))
                }
                Err(e) => Err(e),
            }
        }
    }

    fn parse_legacy_import_member(
        &self,
        member_name: &str,
        coff: &CoffFile<'a>,
    ) -> Result<ImportMember<'a>, MemberParseError> {
        let member_path = Path::new(member_name);

        let symbol_member = LegacyImportSymbolMember::parse(coff)
            .map_err(|e| MemberParseError::new(member_path, e))?;

        let mut imports_cache = self.legacy_imports.borrow_mut();
        let dll = match imports_cache.entry(symbol_member.head_symbol) {
            std::collections::btree_map::Entry::Occupied(dll_entry) => *dll_entry.get(),
            std::collections::btree_map::Entry::Vacant(dll_entry) => {
                // Get the head COFF for this symbol import member
                let head_coff_member = self
                    .extract_archive_member(symbol_member.head_symbol)
                    .map_err(|_| {
                        MemberParseError::new(
                            member_path,
                            MemberParseErrorKind::LegacyImportLibraryMissingSymbol(
                                symbol_member.head_symbol.to_string(),
                            ),
                        )
                    })?;

                let head_coff_data = head_coff_member.data(self.archive_data).map_err(|_| {
                    MemberParseError::new(
                        member_path,
                        MemberParseErrorKind::LegacyImportLibraryMissingSymbol(
                            symbol_member.head_symbol.to_string(),
                        ),
                    )
                })?;

                let head_coff = CoffFile::<&[u8]>::parse(head_coff_data).map_err(|e| {
                    let path = std::str::from_utf8(head_coff_member.name()).unwrap_or(member_name);
                    MemberParseError::new(Path::new(path), e)
                })?;

                let legacy_head_member =
                    LegacyImportHeadMember::parse(&head_coff).map_err(|e| {
                        let path =
                            std::str::from_utf8(head_coff_member.name()).unwrap_or(member_name);
                        MemberParseError::new(Path::new(path), e)
                    })?;

                // Get the tail COFF for the head member.
                let tail_coff_member = self
                    .extract_archive_member(legacy_head_member.tail_symbol)
                    .map_err(|_| {
                        let path =
                            std::str::from_utf8(head_coff_member.name()).unwrap_or(member_name);
                        MemberParseError::new(
                            Path::new(path),
                            MemberParseErrorKind::LegacyImportLibraryMissingSymbol(
                                legacy_head_member.tail_symbol.to_string(),
                            ),
                        )
                    })?;

                let tail_coff_data = tail_coff_member.data(self.archive_data).map_err(|_| {
                    let path = std::str::from_utf8(tail_coff_member.name()).unwrap_or(member_name);
                    MemberParseError::new(
                        Path::new(path),
                        MemberParseErrorKind::LegacyImportLibraryMissingSymbol(
                            symbol_member.head_symbol.to_string(),
                        ),
                    )
                })?;

                let tail_coff = CoffFile::<&[u8]>::parse(tail_coff_data).map_err(|e| {
                    let path = std::str::from_utf8(tail_coff_member.name()).unwrap_or(member_name);
                    MemberParseError::new(Path::new(path), e)
                })?;

                let legacy_tail_member =
                    LegacyImportTailMember::parse(&tail_coff).map_err(|e| {
                        let path =
                            std::str::from_utf8(tail_coff_member.name()).unwrap_or(member_name);
                        MemberParseError::new(Path::new(path), e)
                    })?;

                // Store the mapping from the '_head' symbol found
                // in the symbol COFF to the DLL name found in the
                // '_iname' tail COFF.
                dll_entry.insert(legacy_tail_member.dll)
            }
        };

        Ok(ImportMember {
            architecture: coff.architecture(),
            symbol: symbol_member.public_symbol,
            dll,
            import: symbol_member.import_name,
            typ: symbol_member.typ,
        })
    }

    fn extract_archive_member(
        &self,
        symbol: &'a str,
    ) -> Result<ArchiveMember<'a>, ExtractMemberError> {
        let mut symbol_map = self.symbol_cache.borrow_mut();
        let member_idx = symbol_map
            .find_symbol(symbol)
            .ok_or(ExtractMemberError::NotFound)?;

        self.archive_file
            .member(member_idx)
            .map_err(|e| ExtractMemberError::ArchiveParse(ArchiveParseError::Object(e)))
    }
}

impl<'a> ApiSymbolSource<'a> for PathedItem<&Path, LinkArchive<'a>> {
    fn api_path(&self) -> &Path {
        self.path()
    }

    fn extract_api_symbol(&self, symbol: &'a str) -> Result<ImportMember<'a>, ApiSymbolError> {
        self.deref().extract_api_symbol(symbol)
    }
}

impl<'a> ApiSymbolSource<'a> for LinkArchive<'a> {
    fn extract_api_symbol(&self, symbol: &'a str) -> Result<ImportMember<'a>, ApiSymbolError> {
        let member = match self.extract_archive_member(symbol) {
            Ok(member) => member,
            Err(e) => return Err(e.into()),
        };

        let member_name = std::str::from_utf8(member.name())
            .map_err(|e| ApiSymbolError::ArchiveParse(ArchiveParseError::MemberName(e)))?;

        let member_path = Path::new(member_name);

        let member_data = member
            .data(self.archive_data)
            .map_err(|e| ApiSymbolError::MemberParse(MemberParseError::new(member_path, e)))?;

        if member_data
            .get(..2)
            .is_some_and(|magic| magic == IMAGE_FILE_MACHINE_UNKNOWN.to_le_bytes())
        {
            Ok(ImportFile::parse(member_data)
                .map_err(|e| ApiSymbolError::MemberParse(MemberParseError::new(member_path, e)))?
                .try_into()
                .map_err(|e| ApiSymbolError::MemberParse(MemberParseError::new(member_path, e)))?)
        } else {
            let coff = CoffFile::<&[u8]>::parse(member_data)
                .map_err(|e| ApiSymbolError::MemberParse(MemberParseError::new(member_path, e)))?;

            match self.parse_legacy_import_member(member_name, &coff) {
                Ok(import) => Ok(import),
                Err(e)
                    if matches!(
                        e.kind,
                        MemberParseErrorKind::LegacyImportLibrarySymbolMember(
                            LegacyImportSymbolMemberParseError::Invalid
                        )
                    ) =>
                {
                    Err(ApiSymbolError::ImportMember)
                }
                Err(e) => Err(ApiSymbolError::MemberParse(e)),
            }
        }
    }
}
