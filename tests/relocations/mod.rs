use boflink::linker::LinkerTargetArch;
use object::{Object, ObjectSection, ObjectSymbol, coff::CoffFile};

use crate::link_yaml;

#[test]
fn same_section_flattened() {
    let linked = link_yaml!("same_section_flattened.yaml", LinkerTargetArch::Amd64);
    let coff: CoffFile = CoffFile::parse(linked.as_slice()).expect("Could not parse linked COFF");

    let text_section = coff
        .section_by_name(".text")
        .expect("Could not find .text section in linked COFF");

    assert_eq!(
        text_section
            .coff_section()
            .number_of_relocations
            .get(object::LittleEndian),
        0,
        ".text section header should have 0 for the number of relocations"
    );

    let reloc_count = text_section
        .coff_relocations()
        .expect("Could not get COFF relocations")
        .len();
    assert_eq!(reloc_count, 0, ".text section should have 0 relocations");

    // Check the relocation to see if it was applied properly so that it points
    // to the target symbol
    let target_symbol = coff
        .symbol_by_name("external_function")
        .expect("Could not get external_function symbol");

    let symbol_addr = target_symbol.coff_symbol().value.get(object::LittleEndian);

    let section_data = text_section
        .data()
        .expect("Could not get .text section data");

    let found_reloc_val = u32::from_le_bytes(section_data[2..6].try_into().unwrap());
    let expected_reloc_val = symbol_addr - 2 - 4;

    assert_eq!(
        found_reloc_val, expected_reloc_val,
        "Flattened relocation value does not point to the target symbol"
    );
}

#[test]
fn section_target_shifted() {
    let linked = link_yaml!("section_target_shifted.yaml", LinkerTargetArch::Amd64);
    let coff: CoffFile = CoffFile::parse(linked.as_slice()).expect("Could not parse linked COFF");

    let text_section = coff
        .section_by_name(".text")
        .expect("Could not find .text section in linked COFF");

    let reloc = text_section
        .coff_relocations()
        .expect("Could not get .text section relocation")
        .iter()
        .next()
        .expect(".text section should have a relocation");

    let reloc_addr = reloc.virtual_address.get(object::LittleEndian);

    let section_data = text_section
        .data()
        .expect("Could not get .text section data");

    let found_reloc_val = u32::from_le_bytes(
        section_data[reloc_addr as usize..reloc_addr as usize + 4]
            .try_into()
            .unwrap(),
    );

    assert_eq!(
        found_reloc_val, 16,
        "Relocation value should point to virtual address of shifted section"
    );
}

#[test]
fn defined_symbol_target_no_shift() {
    let linked = link_yaml!(
        "defined_symbol_target_no_shift.yaml",
        LinkerTargetArch::Amd64
    );
    let coff: CoffFile = CoffFile::parse(linked.as_slice()).expect("Could not parse linked COFF");

    let text_section = coff
        .section_by_name(".text")
        .expect("Could not find .text section in linked COFF");

    let reloc = text_section
        .coff_relocations()
        .expect("Could not get .text section relocation")
        .iter()
        .next()
        .expect(".text section should have a relocation");

    let target_symbol = coff
        .symbol_by_index(reloc.symbol())
        .expect("Could not get relocation target symbol");

    let target_name = target_symbol
        .name()
        .expect("Could not get target symbol name");

    assert_eq!(
        target_name, "target_symbol",
        "Relocation target symbol name should be 'target_symbol'"
    );

    let reloc_addr = reloc.virtual_address.get(object::LittleEndian);

    let section_data = text_section
        .data()
        .expect("Could not get .text section data");

    let found_reloc_val = u32::from_le_bytes(
        section_data[reloc_addr as usize..reloc_addr as usize + 4]
            .try_into()
            .unwrap(),
    );

    assert_eq!(
        found_reloc_val, 0,
        "Relocation value should not have shifted"
    );
}
