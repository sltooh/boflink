use boflink::linker::LinkerTargetArch;
use object::{Object, coff::CoffFile};

use crate::link_yaml;

#[test]
fn amd64_empty() {
    let linked = link_yaml!("amd64_empty.yaml", LinkerTargetArch::Amd64);

    let parsed: CoffFile = CoffFile::parse(linked.as_slice()).expect("Could not parse linked COFF");
    assert!(
        parsed.symbol_by_name("go").is_some(),
        "Could not find go symbol in linked COFF"
    );
}

#[test]
fn i386_empty() {
    let linked = link_yaml!("i386_empty.yaml", LinkerTargetArch::I386);

    let parsed: CoffFile = CoffFile::parse(linked.as_slice()).expect("Could not parse linked COFF");
    assert!(
        parsed.symbol_by_name("_go").is_some(),
        "Could not find _go symbol in linked COFF"
    );
}
