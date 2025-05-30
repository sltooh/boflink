use std::path::Path;

use crate::linkobject::import::ImportMember;

use super::error::ApiSymbolError;

pub trait ApiSymbolSource<'a> {
    fn extract_api_symbol(&self, symbol: &'a str) -> Result<ImportMember<'a>, ApiSymbolError>;

    fn api_path(&self) -> &Path {
        Path::new("API")
    }
}
