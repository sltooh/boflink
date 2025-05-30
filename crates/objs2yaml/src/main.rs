use std::{io::BufWriter, path::PathBuf};

use anyhow::Context;
use clap::Parser;
use coffyaml::{
    coff::{
        CoffYaml, CoffYamlAuxFunctionDefinition, CoffYamlAuxSectionDefinition, CoffYamlHeader,
        CoffYamlSection, CoffYamlSectionRelocation, CoffYamlSymbol,
    },
    importlib::ImportlibYaml,
};
use object::{
    Object, ObjectSection, ObjectSymbol,
    coff::{CoffFile, ImageSymbol, ImportFile},
    pe::{IMAGE_SYM_ABSOLUTE, IMAGE_SYM_DEBUG},
    read::archive::ArchiveFile,
};
use serde::Serialize;

#[derive(Parser, Debug)]
#[command(about)]
struct CliArgs {
    /// Input files.
    #[arg(required = true, value_name = "files", value_hint = clap::ValueHint::FilePath)]
    files: Vec<PathBuf>,

    /// Output file. Defaults to stdout.
    #[arg(short, long, value_name = "file", value_hint = clap::ValueHint::FilePath)]
    output: Option<PathBuf>,
}

#[derive(Debug, Serialize)]
enum ParsedInput {
    #[serde(rename = "COFF")]
    Coff(CoffYaml),

    #[serde(rename = "IMPORTLIB")]
    Importlib(ImportlibYaml),
}

fn main() -> anyhow::Result<()> {
    let args = CliArgs::parse();

    let mut parsed_inputs = Vec::with_capacity(args.files.len());

    for file in args.files {
        let data =
            std::fs::read(&file).with_context(|| format!("could not read {}.", file.display()))?;

        if data
            .get(..object::archive::MAGIC.len())
            .is_some_and(|magic| magic == object::archive::MAGIC)
        {
            parsed_inputs
                .push(ParsedInput::Importlib(parse_importlib(data).with_context(
                    || format!("could not parse {}.", file.display()),
                )?));
        } else {
            parsed_inputs.push(ParsedInput::Coff(
                parse_coff(data).with_context(|| format!("could not parse {}.", file.display()))?,
            ));
        }
    }

    let mut output: Box<dyn std::io::Write> = if let Some(filepath) = args.output {
        Box::new(BufWriter::new(
            std::fs::File::create(&filepath)
                .with_context(|| format!("could not open {}.", filepath.display()))?,
        ))
    } else {
        Box::new(BufWriter::new(std::io::stdout().lock()))
    };

    write!(output, "--- ")?;
    let mut ser = serde_yml::Serializer::new(&mut output);
    for parsed in parsed_inputs {
        parsed.serialize(&mut ser)?;
    }

    Ok(())
}

fn parse_importlib(data: Vec<u8>) -> anyhow::Result<ImportlibYaml> {
    let archive = ArchiveFile::parse(data.as_slice())?;

    let first_member = archive.members().nth(3).unwrap()?;
    let member_data = first_member.data(data.as_slice())?;
    let import_file = ImportFile::parse(member_data)?;

    let library = std::str::from_utf8(import_file.dll())?.to_string();

    let mut symbols = Vec::with_capacity(archive.members().count() - 3);

    for member in archive.members().skip(3) {
        let member = member?;
        let member_data = member.data(data.as_slice())?;

        let import_file = ImportFile::parse(member_data)?;
        symbols.push(std::str::from_utf8(import_file.symbol())?.to_string());
    }

    Ok(ImportlibYaml {
        library,
        exports: symbols,
    })
}

fn parse_coff(data: Vec<u8>) -> anyhow::Result<CoffYaml> {
    let coff: CoffFile = CoffFile::parse(data.as_slice())?;

    let coff_header = coff.coff_header();

    let header = CoffYamlHeader {
        machine: coff_header.machine.get(object::LittleEndian),
        characteristics: coff_header.characteristics.get(object::LittleEndian),
    };

    let mut sections = Vec::with_capacity(coff.coff_section_table().len());
    for section in coff.sections() {
        let coff_section = section.coff_section();

        let mut characteristics = coff_section.characteristics.get(object::LittleEndian);
        let alignment = (characteristics & (0xfu32 << 20) != 0)
            .then(|| 2usize.pow((characteristics >> 20 & 0xf) - 1));
        characteristics &= !(0xfu32 << 20);

        let mut relocations = Vec::with_capacity(
            coff_section.number_of_relocations.get(object::LittleEndian) as usize,
        );
        for reloc in section.coff_relocations()? {
            let symbol = coff.symbol_by_index(reloc.symbol())?;

            relocations.push(CoffYamlSectionRelocation {
                symbol_name: symbol.name()?.to_string(),
                virtual_address: reloc.virtual_address.get(object::LittleEndian),
                typ: reloc.typ.get(object::LittleEndian),
            });
        }

        sections.push(CoffYamlSection {
            name: section.name()?.to_string(),
            characteristics,
            alignment,
            section_data: section.data()?.to_vec(),
            size_of_raw_data: Some(coff_section.size_of_raw_data.get(object::LittleEndian)),
            relocations,
        });
    }

    let symbol_table = coff.coff_symbol_table();
    let mut symbols = Vec::with_capacity(symbol_table.len());

    for symbol in coff.symbols() {
        let coff_symbol = symbol.coff_symbol();

        let section_definition = if coff_symbol.has_aux_section() {
            let aux_section = symbol_table.aux_section(symbol.index())?;
            Some(CoffYamlAuxSectionDefinition {
                length: aux_section.length.get(object::LittleEndian),
                number_of_relocations: aux_section.number_of_relocations.get(object::LittleEndian),
                number_of_linenumbers: aux_section.number_of_linenumbers.get(object::LittleEndian),
                check_sum: aux_section.check_sum.get(object::LittleEndian),
                number: aux_section.number.get(object::LittleEndian),
                selection: aux_section.selection,
            })
        } else {
            None
        };

        let function_definition = if coff_symbol.has_aux_function() {
            let aux_function = symbol_table.aux_function(symbol.index())?;
            Some(CoffYamlAuxFunctionDefinition {
                tag_index: aux_function.tag_index.get(object::LittleEndian),
                total_size: aux_function.total_size.get(object::LittleEndian),
                pointer_to_linenumber: aux_function.pointer_to_linenumber.get(object::LittleEndian),
                pointer_to_next_function: aux_function
                    .pointer_to_next_function
                    .get(object::LittleEndian),
            })
        } else {
            None
        };

        let file = if coff_symbol.has_aux_file_name() {
            Some(symbol.name()?.to_string())
        } else {
            None
        };

        symbols.push(CoffYamlSymbol {
            name: if coff_symbol.has_aux_file_name() {
                ".file".to_string()
            } else {
                symbol.name()?.to_string()
            },
            value: coff_symbol.value.get(object::LittleEndian),
            section_number: match coff_symbol.section_number.get(object::LittleEndian) {
                0xffff => IMAGE_SYM_ABSOLUTE,
                0xfffe => IMAGE_SYM_DEBUG,
                o => o.into(),
            },
            simple_type: coff_symbol.base_type(),
            complex_type: coff_symbol.derived_type(),
            storage_class: coff_symbol.storage_class,
            section_definition,
            function_definition,
            file,
        });
    }

    Ok(CoffYaml {
        header,
        sections,
        symbols,
    })
}
