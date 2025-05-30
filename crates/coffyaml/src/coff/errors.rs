use std::num::TryFromIntError;

#[derive(Debug, thiserror::Error)]
pub enum CoffYamlCoffBuildError {
    #[error("{0}")]
    IntegerConversion(#[from] TryFromIntError),

    #[error("relocation target symbol {0} does not exist")]
    MissingSymbol(String),

    #[error("alignment value of {align} for section index {index} is not valid")]
    SectionAlign { index: usize, align: usize },

    #[error("{0}")]
    ObjectWrite(#[from] object::write::Error),
}
