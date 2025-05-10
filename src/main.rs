use std::path::PathBuf;

use clap::Parser;
use eraro::Eraro;

mod eraro;
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
    },
}

impl Command {
    fn execute(self) -> Result<(), Eraro> {
        match self {
            Command::Embed {
                kovrilodosiero,
                dosierenhavo,
                stegodosiero,
            } => komuna::ensxipigxi(kovrilodosiero, dosierenhavo, stegodosiero),
            Command::Extract {
                stegodosiero,
                celloko,
            } => komuna::ekstrakti(stegodosiero, celloko),
        }
    }
}

fn main() -> Result<(), Eraro> {
    Args::parse().ordo.execute()
}
