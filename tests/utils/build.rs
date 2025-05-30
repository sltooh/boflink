use coffyaml::{coff::CoffYaml, importlib::ImportlibYaml};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum YamlInput {
    #[serde(rename = "COFF")]
    Coff(CoffYaml),

    #[serde(rename = "IMPORTLIB")]
    Importlib(ImportlibYaml),
}
