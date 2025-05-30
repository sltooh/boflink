# jamcrc
JamCRC wrapper around the [`crc32fast`](https://github.com/srijs/rust-crc32fast) crate.
Used for calculating COFF auxiliary section checksums.

This is split up into a separate crate to expose it as a command line tool [`jamcrc-cli`](cli).

## Command line usage
```shell
cargo r -p jamcrc-cli -- [arguments]
cargo r -p jamcrc-cli -- -h
```

### Usage
```
Usage: jamcrc-cli [OPTIONS] [FILE]

Arguments:
  [FILE]  Input file to calculate the checksum for. Use "-" to read from stdin

Options:
  -s, --string <string>  Input string to calculate the checksum for instead of a file
  -i, --init <INIT>      Init value for the calcuation [default: 0]
      --ihex             Decode the passed in input as hex
      --hex              Print the calculated checksum as hex
  -h, --help             Print help
```
