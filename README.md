# Smaragdina

Steganography tool. Inspired by [Steghide](https://steghide.sourceforge.net/).

Hide files in images.

Relying on [XChaCha20Poly1305](https://crates.io/crates/chacha20poly1305) for embedded data encryption and [Blake2](https://crates.io/crates/blake2) for passphrase derivation.

# Build

> cargo build --release

# Usage

```
smaragdina <COMMAND>

Commands:
  embed    
  extract  
  help     Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

## Embed

```
smaragdina embed [OPTIONS] --cover_file <KOVRILODOSIERO> --embed_file <DOSIERENHAVO> --stego_file <STEGODOSIERO>

Options:
  -c, --cover_file <KOVRILODOSIERO>  Path to the cover file in which to embed data
  -e, --embed_file <DOSIERENHAVO>    Path to the data file to be hidden
  -p, --passphrase <SEKRETA_FRAZO>   Passphrase used for encryption, won't be encrypted otherwise
  -h, --hint <SUGESTO>               Hint for the passphrase
  -s, --stego_file <STEGODOSIERO>    Path to the stego file (containing hidden data) for extraction
  -h, --help                         Print help
```

## Extract

```
smaragdina extract [OPTIONS] --stego_file <STEGODOSIERO>

Options:
  -s, --stego_file <STEGODOSIERO>   Path to the stego file (containing hidden data) for extraction
  -d, --dest <CELLOKO>              Path to directory where the embed tile will be extracted
  -p, --passphrase <SEKRETA_FRAZO>  Passphrase used for decryption
  -h, --help
```

# Side node

The code is written in Esperanto for reasons I don't feel the need to justify.

# TODO

Hide files in sound files using [Hound](https://crates.io/crates/hound).