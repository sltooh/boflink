use std::path::PathBuf;

use crate::{
    api::ApiSymbolError,
    graph::{LinkGraphAddError, LinkGraphLinkError},
    libsearch::LibsearchError,
    linkobject::archive::{ArchiveParseError, LinkArchiveParseError, MemberParseErrorKind},
};

#[derive(Debug, thiserror::Error)]
pub enum LinkError {
    #[error("{0}")]
    Setup(LinkerSetupErrors),

    #[error("{0}")]
    Symbol(LinkerSymbolErrors),

    #[error("{0}")]
    Graph(#[from] LinkGraphLinkError),

    #[error("no input files")]
    NoInput,

    #[error("could not detect architecture")]
    ArchitectureDetect,
}

#[derive(Debug, thiserror::Error)]
#[error("{}", display_vec(.0))]
pub struct LinkerSetupErrors(pub(super) Vec<LinkerSetupError>);

impl LinkerSetupErrors {
    pub fn errors(&self) -> &[LinkerSetupError] {
        &self.0
    }
}

#[derive(Debug, thiserror::Error)]
pub enum LinkerSetupError {
    #[error("{0}")]
    Path(LinkerSetupPathError),

    #[error("{0}")]
    Library(LibsearchError),

    #[error("{0}")]
    ApiInit(ApiInitError),
}

#[derive(Debug, thiserror::Error)]
pub enum ApiInitError {
    #[error("{}: could not open custom API: {error}", .path.display())]
    Io {
        path: PathBuf,
        error: std::io::Error,
    },

    #[error("unable to find custom API '{0}'")]
    NotFound(String),

    #[error("{}: {error}", .path.display())]
    Parse {
        path: PathBuf,
        error: LinkArchiveParseError,
    },
}

impl From<LibsearchError> for ApiInitError {
    fn from(value: LibsearchError) -> Self {
        match value {
            LibsearchError::NotFound(name) => Self::NotFound(name),
            LibsearchError::Io { path, error } => Self::Io { path, error },
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error(
    "{}{}: {error}",
    .path.display(),
    .member.as_ref().map(|p| format!("({})", p.display())).unwrap_or_default()
)]
pub struct LinkerSetupPathError {
    pub path: PathBuf,
    pub member: Option<PathBuf>,
    pub error: LinkerPathErrorKind,
}

impl LinkerSetupPathError {
    pub fn new<P: Into<PathBuf>>(
        path: impl Into<PathBuf>,
        member: Option<P>,
        error: impl Into<LinkerPathErrorKind>,
    ) -> LinkerSetupPathError {
        Self {
            path: path.into(),
            member: member.map(Into::into),
            error: error.into(),
        }
    }

    pub fn nomember(
        path: impl Into<PathBuf>,
        error: impl Into<LinkerPathErrorKind>,
    ) -> LinkerSetupPathError {
        Self {
            path: path.into(),
            member: None,
            error: error.into(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum LinkerPathErrorKind {
    #[error("{0}")]
    DrectveLibrary(#[from] DrectveLibsearchError),

    #[error("{0}")]
    ArchiveParse(#[from] LinkArchiveParseError),

    #[error("{0}")]
    ArchiveExtract(#[from] ArchiveParseError),

    #[error("{0}")]
    MemberExtract(#[from] MemberParseErrorKind),

    #[error("{0}")]
    GraphAdd(#[from] LinkGraphAddError),

    #[error("{0}")]
    ApiSymbol(#[from] ApiSymbolError),

    #[error("{0}")]
    Object(#[from] object::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum DrectveLibsearchError {
    #[error("unable to find library {0}")]
    NotFound(String),

    #[error("could not open link library {}: {error}", .path.display())]
    Io {
        path: PathBuf,
        error: std::io::Error,
    },
}

impl From<LibsearchError> for DrectveLibsearchError {
    fn from(value: LibsearchError) -> Self {
        match value {
            LibsearchError::Io { path, error } => Self::Io { path, error },
            LibsearchError::NotFound(name) => Self::NotFound(name),
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("{}", display_vec(.0))]
pub struct LinkerSymbolErrors(pub(super) Vec<String>);

impl LinkerSymbolErrors {
    pub fn errors(&self) -> &[String] {
        &self.0
    }
}

struct DisplayVec<'a, T: std::fmt::Display>(&'a Vec<T>);

impl<'a, T: std::fmt::Display> std::fmt::Display for DisplayVec<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut value_iter = self.0.iter();

        let first_value = match value_iter.next() {
            Some(v) => v,
            None => return Ok(()),
        };

        first_value.fmt(f)?;

        for val in value_iter {
            write!(f, "\n{val}")?;
        }

        Ok(())
    }
}

fn display_vec<T: std::fmt::Display>(errors: &Vec<T>) -> DisplayVec<'_, T> {
    DisplayVec(errors)
}
