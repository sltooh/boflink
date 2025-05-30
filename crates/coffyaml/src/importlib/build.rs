use object::pe::{
    IMAGE_FILE_MACHINE_UNKNOWN, IMAGE_SCN_CNT_INITIALIZED_DATA, IMAGE_SCN_MEM_READ,
    IMAGE_SCN_MEM_WRITE, IMAGE_SYM_CLASS_EXTERNAL, IMAGE_SYM_CLASS_SECTION, IMAGE_SYM_CLASS_STATIC,
    IMPORT_OBJECT_CODE, IMPORT_OBJECT_NAME, IMPORT_OBJECT_NAME_MASK, IMPORT_OBJECT_NAME_SHIFT,
    IMPORT_OBJECT_TYPE_MASK,
};

use crate::{
    archive::builder::ArchiveBuilder,
    coff::{CoffYaml, CoffYamlHeader, CoffYamlSection, CoffYamlSectionRelocation, CoffYamlSymbol},
};

use super::{Architecture, ArchitectureConfig, ImportlibYaml, errors::ImportlibYamlBuildError};

impl ImportlibYaml {
    pub fn build(self, arch: Architecture) -> Result<Vec<u8>, ImportlibYamlBuildError> {
        let cfg = ArchitectureConfig::new(arch)?;

        // Import descriptor, NULL import descriptor, NULL thunk data, import members
        let member_count = 3 + self.exports.len();

        let mut archive_builder = ArchiveBuilder::msvc_archive_with_capacity(member_count);

        // The library name for the import descriptor symbols
        let library_name = self
            .library
            .rsplit_once('.')
            .and_then(|(prefix, suffix)| {
                suffix
                    .eq_ignore_ascii_case("dll")
                    .then(|| prefix.to_string())
            })
            .unwrap_or_else(|| self.library.clone());

        let import_descriptor_name = format!("__IMPORT_DESCRIPTOR_{library_name}");
        let null_import_descriptor_name = "__NULL_IMPORT_DESCRIPTOR";
        let null_thunk_data_name = format!("\x7f{library_name}_NULL_THUNK_DATA");

        // Add the import descriptor member
        let mut member = archive_builder.add_member(
            &self.library,
            CoffYaml {
                header: CoffYamlHeader {
                    machine: cfg.machine(),
                    characteristics: 0,
                },
                sections: vec![
                    CoffYamlSection {
                        name: ".idata$2".to_string(),
                        characteristics: IMAGE_SCN_CNT_INITIALIZED_DATA
                            | IMAGE_SCN_MEM_READ
                            | IMAGE_SCN_MEM_WRITE,
                        alignment: Some(4),
                        section_data: vec![0u8; 20],
                        size_of_raw_data: None,
                        relocations: vec![
                            CoffYamlSectionRelocation {
                                virtual_address: 12,
                                symbol_name: ".idata$6".to_string(),
                                typ: cfg.reloc_type(),
                            },
                            CoffYamlSectionRelocation {
                                virtual_address: 0,
                                symbol_name: ".idata$4".to_string(),
                                typ: cfg.reloc_type(),
                            },
                            CoffYamlSectionRelocation {
                                virtual_address: 16,
                                symbol_name: ".idata$5".to_string(),
                                typ: cfg.reloc_type(),
                            },
                        ],
                    },
                    CoffYamlSection {
                        name: ".idata$6".to_string(),
                        characteristics: IMAGE_SCN_CNT_INITIALIZED_DATA
                            | IMAGE_SCN_MEM_READ
                            | IMAGE_SCN_MEM_WRITE,
                        alignment: Some(2),
                        section_data: format!("{}\0", &self.library).as_bytes().to_vec(),
                        ..Default::default()
                    },
                ],
                symbols: vec![
                    CoffYamlSymbol {
                        name: import_descriptor_name.clone(),
                        section_number: 1,
                        storage_class: IMAGE_SYM_CLASS_EXTERNAL,
                        ..Default::default()
                    },
                    CoffYamlSymbol {
                        name: ".idata$2".to_string(),
                        section_number: 1,
                        storage_class: IMAGE_SYM_CLASS_SECTION,
                        ..Default::default()
                    },
                    CoffYamlSymbol {
                        name: ".idata$6".to_string(),
                        section_number: 2,
                        storage_class: IMAGE_SYM_CLASS_STATIC,
                        ..Default::default()
                    },
                    CoffYamlSymbol {
                        name: ".idata$4".to_string(),
                        storage_class: IMAGE_SYM_CLASS_SECTION,
                        ..Default::default()
                    },
                    CoffYamlSymbol {
                        name: ".idata$5".to_string(),
                        storage_class: IMAGE_SYM_CLASS_SECTION,
                        ..Default::default()
                    },
                    CoffYamlSymbol {
                        name: null_import_descriptor_name.to_string(),
                        storage_class: IMAGE_SYM_CLASS_EXTERNAL,
                        ..Default::default()
                    },
                    CoffYamlSymbol {
                        name: null_thunk_data_name.clone(),
                        storage_class: IMAGE_SYM_CLASS_EXTERNAL,
                        ..Default::default()
                    },
                ],
            }
            .build()
            .unwrap(),
        );
        member.date(0);
        member.uid(0);
        member.gid(0);
        member.mode(644);
        member.export(&import_descriptor_name);

        // Add the NULL import descriptor member
        let mut member = archive_builder.add_member(
            &self.library,
            CoffYaml {
                header: CoffYamlHeader {
                    machine: cfg.machine(),
                    characteristics: 0,
                },
                sections: vec![CoffYamlSection {
                    name: ".idata$3".to_string(),
                    characteristics: IMAGE_SCN_CNT_INITIALIZED_DATA
                        | IMAGE_SCN_MEM_READ
                        | IMAGE_SCN_MEM_WRITE,
                    alignment: Some(4),
                    section_data: vec![0u8; 20],
                    ..Default::default()
                }],
                symbols: vec![CoffYamlSymbol {
                    name: null_import_descriptor_name.to_string(),
                    section_number: 1,
                    storage_class: IMAGE_SYM_CLASS_EXTERNAL,
                    ..Default::default()
                }],
            }
            .build()
            .unwrap(),
        );
        member.date(0);
        member.uid(0);
        member.gid(0);
        member.mode(644);
        member.export(null_import_descriptor_name);

        // Add the NULL thunk data member
        let mut member = archive_builder.add_member(
            &self.library,
            CoffYaml {
                header: CoffYamlHeader {
                    machine: cfg.machine(),
                    characteristics: 0,
                },
                sections: vec![
                    CoffYamlSection {
                        name: ".idata$5".to_string(),
                        characteristics: IMAGE_SCN_CNT_INITIALIZED_DATA
                            | IMAGE_SCN_MEM_READ
                            | IMAGE_SCN_MEM_WRITE,
                        alignment: Some(8),
                        section_data: vec![0u8; 8],
                        ..Default::default()
                    },
                    CoffYamlSection {
                        name: ".idata$4".to_string(),
                        characteristics: IMAGE_SCN_CNT_INITIALIZED_DATA
                            | IMAGE_SCN_MEM_READ
                            | IMAGE_SCN_MEM_WRITE,
                        alignment: Some(8),
                        section_data: vec![0u8; 8],
                        ..Default::default()
                    },
                ],
                symbols: vec![CoffYamlSymbol {
                    name: null_thunk_data_name.clone(),
                    section_number: 1,
                    storage_class: IMAGE_SYM_CLASS_EXTERNAL,
                    ..Default::default()
                }],
            }
            .build()
            .unwrap(),
        );
        member.date(0);
        member.uid(0);
        member.gid(0);
        member.mode(644);
        member.export(null_thunk_data_name);

        // Add each import COFF
        for export in self.exports {
            let mut member = archive_builder.add_member(
                &self.library,
                build_import_coff(cfg.machine(), &export, &self.library),
            );
            member.date(0);
            member.uid(0);
            member.gid(0);
            member.mode(644);
            member.exports([format!("__imp_{}", &export), export]);
        }

        Ok(archive_builder.build())
    }
}

fn build_import_coff(machine: u16, export: impl AsRef<str>, dllname: impl AsRef<str>) -> Vec<u8> {
    let mut buffer =
        Vec::with_capacity(20 + export.as_ref().len() + 1 + dllname.as_ref().len() + 1);

    // Header
    buffer.extend(IMAGE_FILE_MACHINE_UNKNOWN.to_le_bytes()); // Sig1
    buffer.extend(0xffffu16.to_le_bytes()); // Sig2
    buffer.extend(0u16.to_le_bytes()); // Version
    buffer.extend(machine.to_le_bytes()); // Machine
    buffer.extend(0u32.to_le_bytes()); // Time-Date Stamp

    // Size of data
    buffer.extend(((export.as_ref().len() + 1 + dllname.as_ref().len() + 1) as u32).to_le_bytes());

    // Ordinal/Hint
    buffer.extend(0u16.to_le_bytes());

    // Import metadata
    let mut meta: u16 = 0;

    // Code import type
    meta |= IMPORT_OBJECT_CODE & IMPORT_OBJECT_TYPE_MASK;

    // Import by name
    meta |= (IMPORT_OBJECT_NAME & IMPORT_OBJECT_NAME_MASK) << IMPORT_OBJECT_NAME_SHIFT;

    buffer.extend(meta.to_le_bytes());

    // Import name string
    buffer.extend(export.as_ref().as_bytes());
    buffer.push(0);

    // DLL name string
    buffer.extend(dllname.as_ref().as_bytes());
    buffer.push(0);

    buffer
}
