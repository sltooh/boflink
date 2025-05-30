use coffyaml::importlib::{Architecture, ImportlibYaml};
use object::{
    coff::{ImportFile, ImportName},
    read::archive::ArchiveFile,
};

const IMPORTLIB_YAML: &str = include_str!("importlib.yaml");

#[test]
fn importlib_sanity_parse() {
    assert!(serde_yml::from_str::<ImportlibYaml>(IMPORTLIB_YAML).is_ok());
}

#[test]
fn importlib_sanity_build() {
    let parsed: ImportlibYaml = serde_yml::from_str(IMPORTLIB_YAML).unwrap();
    assert!(parsed.build(Architecture::X86_64).is_ok());
}

#[test]
fn importlib_sanity_object_can_parse() {
    let parsed_yaml: ImportlibYaml = serde_yml::from_str(IMPORTLIB_YAML).unwrap();
    let built = parsed_yaml.build(Architecture::X86_64).unwrap();
    assert!(ArchiveFile::parse(built.as_slice()).is_ok());
}

#[test]
fn importlib_symbol_table_exports() {
    let parsed_yaml: ImportlibYaml = serde_yml::from_str(IMPORTLIB_YAML).unwrap();

    let exports_list = parsed_yaml.exports.clone();

    let built = parsed_yaml.build(Architecture::X86_64).unwrap();
    let parsed_archive = ArchiveFile::parse(built.as_slice()).unwrap();

    let archive_symbols = parsed_archive.symbols().unwrap().unwrap();

    for export in exports_list {
        assert!(
            archive_symbols
                .clone()
                .any(|symbol| std::str::from_utf8(symbol.unwrap().name()).unwrap() == export),
            "could not find '{export}' in symbol table"
        );
    }
}

#[test]
fn importlib_extract_member() {
    let parsed_yaml: ImportlibYaml = serde_yml::from_str(IMPORTLIB_YAML).unwrap();

    let built = parsed_yaml.build(Architecture::X86_64).unwrap();
    let parsed_archive = ArchiveFile::parse(built.as_slice()).unwrap();

    let mut archive_symbols = parsed_archive.symbols().unwrap().unwrap();
    let symbol = archive_symbols
        .find(|symbol| std::str::from_utf8(symbol.unwrap().name()).unwrap() == "ExportedSymbol")
        .unwrap()
        .unwrap();

    let extracted_member = parsed_archive.member(symbol.offset()).unwrap();
    let extracted_data = extracted_member.data(built.as_slice()).unwrap();

    let import_file = ImportFile::parse(extracted_data).unwrap();
    match import_file.import() {
        ImportName::Name(s) => {
            let name = std::str::from_utf8(s).unwrap();
            assert_eq!(
                name, "ExportedSymbol",
                "extracted import member public import name does not match name in the symbol map"
            );
        }
        ImportName::Ordinal(_) => panic!("import value should not be an ordinal"),
    }
}
