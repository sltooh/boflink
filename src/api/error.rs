use crate::linkobject::archive::{ArchiveParseError, ExtractMemberError, MemberParseError};

#[derive(Debug, thiserror::Error)]
pub enum ApiSymbolError {
    #[error("member for symbol does not exist")]
    NotFound,

    #[error("{0}")]
    ArchiveParse(ArchiveParseError),

    #[error("{0}")]
    MemberParse(MemberParseError),

    #[error("invalid COFF import library member")]
    ImportMember,
}

impl From<ExtractMemberError> for ApiSymbolError {
    fn from(value: ExtractMemberError) -> Self {
        match value {
            ExtractMemberError::NotFound => Self::NotFound,
            ExtractMemberError::ArchiveParse(e) => Self::ArchiveParse(e),
            ExtractMemberError::MemberParse(e) => Self::MemberParse(e),
        }
    }
}
