use std::path::PathBuf;

use clap::Parser;

mod komuna;
mod sekreto;
mod servajxoj;

// https://linuxcommandlibrary.com/man/steghide
// https://jsmnbom.github.io/toki-pona-helper/sitelenPonaConverter.html

/// Smaragdina
#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    ordo: Command,
}

#[derive(Parser)]
enum Command {
    Embed {
        /// Path to the cover file in which to embed data
        #[arg(short = 'c', long = "cover_file")]
        kovrilodosiero: PathBuf,
        /// Path to the data file to be hidden
        #[arg(short = 'e', long = "embed_file")]
        dosierenhavo: PathBuf,
        /// Passphrase used for encryption, won't be encrypted otherwise
        #[arg(short = 'p', long = "passphrase")]
        sekreta_frazo: Option<String>,
        /// Hint for the passphrase
        #[arg(short = 'h', long = "hint")]
        sugesto: Option<String>,
        /// Path to the stego file (containing hidden data) for extraction
        #[arg(short = 's', long = "stego_file")]
        stegodosiero: PathBuf,
    },
    Extract {
        /// Path to the stego file (containing hidden data) for extraction
        #[arg(short = 's', long = "stego_file")]
        stegodosiero: PathBuf,
        /// Path to directory where the embed tile will be extracted
        #[arg(short = 'd', long = "dest")]
        celloko: Option<PathBuf>,
        /// Passphrase used for decryption
        #[arg(short = 'p', long = "passphrase")]
        sekreta_frazo: Option<String>,
    },
}

impl Command {
    fn execute(self) -> Result<(), komuna::Eraro> {
        match self {
            Command::Embed {
                kovrilodosiero,
                dosierenhavo,
                sekreta_frazo,
                sugesto,
                stegodosiero,
            } => komuna::ensxipigxi(
                kovrilodosiero,
                dosierenhavo,
                stegodosiero,
                match (sugesto, sekreta_frazo) {
                    (None, None) => None,
                    (None, Some(sekreta_frazo)) => Some((String::new(), sekreta_frazo)),
                    (Some(_), None) => panic!("Missing passphrase"),
                    (Some(sugesto), Some(sekreta_frazo)) => Some((sugesto, sekreta_frazo)),
                },
            ),
            Command::Extract {
                stegodosiero,
                celloko,
                sekreta_frazo,
            } => komuna::ekstrakti(stegodosiero, celloko, sekreta_frazo),
        }
    }
}

fn main() -> Result<(), komuna::Eraro> {
    Args::parse().ordo.execute()
}
