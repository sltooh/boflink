use super::Architecture;

#[derive(Debug, thiserror::Error)]
pub enum ImportlibYamlBuildError {
    #[error("architecture {0:?} is not supported")]
    UnsupportArchitecture(Architecture),
}
