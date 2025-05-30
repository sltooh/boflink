use coffyaml::coff::CoffYaml;
use object::{coff::CoffFile, pe};

const COFF_YAML: &str = include_str!("coff.yaml");

#[test]
fn coff_sanity_parse() {
    assert!(serde_yml::from_str::<CoffYaml>(COFF_YAML).is_ok());
}

#[test]
fn coff_sanity_build() {
    let parsed: CoffYaml = serde_yml::from_str(COFF_YAML).unwrap();
    assert!(parsed.build().is_ok());
}

#[test]
fn coff_sanity_object_can_parse() {
    let parsed_yaml: CoffYaml = serde_yml::from_str(COFF_YAML).unwrap();
    let built = parsed_yaml.build().unwrap();
    assert!(CoffFile::<_, pe::ImageFileHeader>::parse(built.as_slice()).is_ok());
}
