use std::collections::HashMap;

use errors::CoffYamlCoffBuildError;
use object::{
    pe::{IMAGE_SYM_ABSOLUTE, IMAGE_SYM_DEBUG, IMAGE_SYM_DTYPE_SHIFT, IMAGE_SYM_UNDEFINED},
    write::coff::{AuxSymbolSection, FileHeader, Relocation, SectionHeader, Symbol, Writer},
};
use serde::{Deserialize, Serialize};

pub mod errors;
mod header;
mod sections;
mod symbols;

pub use header::CoffYamlHeader;
pub use sections::{CoffYamlSection, CoffYamlSectionRelocation};
pub use symbols::{CoffYamlAuxFunctionDefinition, CoffYamlAuxSectionDefinition, CoffYamlSymbol};

const SECTION_ALIGN_SHIFT: u32 = 20;

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct CoffYaml {
    pub header: CoffYamlHeader,
    pub sections: Vec<CoffYamlSection>,
    pub symbols: Vec<CoffYamlSymbol>,
}

impl CoffYaml {
    pub fn build(mut self) -> Result<Vec<u8>, CoffYamlCoffBuildError> {
        let mut buffer = Vec::new();

        let mut writer = Writer::new(&mut buffer);
        writer.reserve_file_header();
        writer.reserve_section_headers(self.sections.len().try_into()?);

        let mut section_headers = Vec::with_capacity(self.sections.len());
        for (idx, section) in self.sections.iter().enumerate() {
            let alignment_flag = if let Some(alignment) = section.alignment {
                if alignment == 0 || alignment > 8192 || (alignment != 1 && alignment % 2 != 0) {
                    return Err(CoffYamlCoffBuildError::SectionAlign {
                        index: idx,
                        align: alignment,
                    });
                }

                ((alignment as u32).ilog2() + 1) << SECTION_ALIGN_SHIFT
            } else {
                0
            };

            section_headers.push(SectionHeader {
                name: writer.add_name(section.name.as_bytes()),
                size_of_raw_data: if let Some(size) = section.size_of_raw_data {
                    size
                } else {
                    section.section_data.len().try_into()?
                },
                pointer_to_raw_data: writer.reserve_section(section.section_data.len()),
                pointer_to_relocations: 0,
                pointer_to_linenumbers: 0,
                number_of_relocations: section.relocations.len().try_into()?,
                number_of_linenumbers: 0,
                characteristics: section.characteristics | alignment_flag,
            });
        }

        for (section_header, section) in section_headers.iter_mut().zip(self.sections.iter()) {
            section_header.pointer_to_relocations =
                writer.reserve_relocations(section.relocations.len());
        }

        let mut symbol_map = HashMap::with_capacity(self.symbols.len());
        let mut symbol_names = Vec::with_capacity(self.symbols.len());

        for symbol in self.symbols.iter_mut() {
            if let Some(aux_file) = symbol.file.as_mut() {
                aux_file.truncate(18);
            }
        }

        for symbol in &self.symbols {
            let idx = writer.reserve_symbol_index();
            symbol_names.push(writer.add_name(symbol.name.as_bytes()));
            symbol_map.insert(&symbol.name, idx);

            if let Some(aux_file) = symbol.file.as_ref() {
                writer.reserve_aux_file_name(aux_file.as_bytes());
            }

            if symbol.section_definition.as_ref().is_some() {
                writer.reserve_aux_section();
            }
        }

        writer.reserve_symtab_strtab();

        writer.write_file_header(FileHeader {
            machine: self.header.machine,
            time_date_stamp: 0,
            characteristics: self.header.characteristics,
        })?;

        for header in section_headers {
            writer.write_section_header(header);
        }

        for section in &self.sections {
            writer.write_section(&section.section_data);
        }

        for section in &self.sections {
            if section.relocations.len() > u16::MAX as usize {
                writer.write_relocations_count(section.relocations.len());
            }

            for reloc in &section.relocations {
                writer.write_relocation(Relocation {
                    virtual_address: reloc.virtual_address,
                    symbol: symbol_map.get(&reloc.symbol_name).copied().ok_or_else(|| {
                        CoffYamlCoffBuildError::MissingSymbol(reloc.symbol_name.clone())
                    })?,
                    typ: reloc.typ,
                });
            }
        }

        for (symbol_name, symbol) in symbol_names.into_iter().zip(self.symbols.iter()) {
            let aux_count = if symbol.file.is_some() { 1 } else { 0 }
                + if symbol.section_definition.is_some() {
                    1
                } else {
                    0
                };

            writer.write_symbol(Symbol {
                name: symbol_name,
                value: symbol.value,
                section_number: match symbol.section_number {
                    IMAGE_SYM_UNDEFINED => 0,
                    IMAGE_SYM_ABSOLUTE => u16::MAX,
                    IMAGE_SYM_DEBUG => u16::MAX - 1,
                    _ => symbol.section_number.try_into()?,
                },
                typ: (symbol.complex_type << IMAGE_SYM_DTYPE_SHIFT) | (symbol.simple_type & 0xff),
                number_of_aux_symbols: aux_count,
                storage_class: symbol.storage_class,
            });

            if let Some(aux_file) = symbol.file.as_ref() {
                writer.write_aux_file_name(aux_file.as_bytes(), 1);
            }

            if let Some(aux_section) = symbol.section_definition.as_ref() {
                writer.write_aux_section(AuxSymbolSection {
                    length: aux_section.length,
                    number_of_relocations: aux_section.number_of_relocations.into(),
                    number_of_linenumbers: aux_section.number_of_linenumbers,
                    check_sum: aux_section.check_sum,
                    number: aux_section.number.into(),
                    selection: aux_section.selection,
                });
            }
        }

        writer.write_strtab();

        Ok(buffer)
    }
}
