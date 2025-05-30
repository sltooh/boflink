use std::{
    io::{BufRead, BufReader},
    path::PathBuf,
};

use clap::Parser;
use hexstream::HexDecodeStream;

mod hexstream;

#[derive(Parser, Debug)]
#[command(about)]
struct CliArgs {
    /// Input file to calculate the checksum for. Use "-" to read from stdin
    #[arg(value_parser = parse_stdin_or_filepath)]
    file: Option<StdinOrFilePath>,

    /// Input string to calculate the checksum for instead of a file
    #[arg(
        id = "string",
        long,
        short,
        conflicts_with = "file",
        default_value = "",
        hide_default_value = true
    )]
    input_string: String,

    /// Init value for the calcuation
    #[arg(long, short, default_value = "0", allow_negative_numbers = true)]
    init: i32,

    /// Decode the passed in input as hex
    #[arg(long)]
    ihex: bool,

    /// Print the calculated checksum as hex
    #[arg(long)]
    hex: bool,
}

#[derive(Clone, Debug)]
enum StdinOrFilePath {
    Stdin,
    FilePath(PathBuf),
}

fn parse_stdin_or_filepath(
    argval: &str,
) -> Result<StdinOrFilePath, Box<dyn std::error::Error + Send + Sync>> {
    match argval {
        "-" => Ok(StdinOrFilePath::Stdin),
        _ => Ok(StdinOrFilePath::FilePath(PathBuf::from(argval))),
    }
}

fn calculate_buffered<R: BufRead>(
    mut hasher: jamcrc::Hasher,
    mut reader: R,
) -> anyhow::Result<u32> {
    loop {
        let buffer = reader.fill_buf()?;
        if buffer.is_empty() {
            return Ok(hasher.finalize());
        }

        let consumed = buffer.len();
        hasher.update(buffer);
        reader.consume(consumed);
    }
}

fn calculate_full(mut hasher: jamcrc::Hasher, data: impl AsRef<[u8]>) -> u32 {
    hasher.update(data.as_ref());
    hasher.finalize()
}

fn main() -> anyhow::Result<()> {
    let args = CliArgs::parse();

    let hasher = jamcrc::Hasher::new_with_initial(args.init.cast_unsigned());

    let checksum = if let Some(file) = args.file.as_ref() {
        match file {
            StdinOrFilePath::Stdin => {
                if args.ihex {
                    calculate_buffered(
                        hasher,
                        BufReader::new(HexDecodeStream::new(std::io::stdin().lock())),
                    )?
                } else {
                    calculate_buffered(hasher, std::io::stdin().lock())?
                }
            }
            StdinOrFilePath::FilePath(path) => {
                let f = std::fs::File::open(path)?;
                if args.ihex {
                    calculate_buffered(hasher, BufReader::new(HexDecodeStream::new(f)))?
                } else {
                    calculate_buffered(hasher, BufReader::new(f))?
                }
            }
        }
    } else if args.ihex {
        calculate_buffered(
            hasher,
            BufReader::new(HexDecodeStream::new(args.input_string.as_bytes())),
        )?
    } else {
        calculate_full(hasher, args.input_string.as_bytes())
    };

    if args.hex {
        println!("{checksum:#x}");
    } else {
        println!("{checksum}");
    }

    Ok(())
}
