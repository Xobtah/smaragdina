use std::{io, path::PathBuf};

use clap::Parser;
use image::ImageReader;

const NULL_PIXEL: image::Rgba<u8> = image::Rgba([0, 0, 0, 0]);

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Io error: {0}")]
    Io(#[from] io::Error),
    #[error("Image error: {0}")]
    Image(#[from] image::error::ImageError),
}

/// Smaragdina
#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Parser)]
enum Command {
    Hide {
        message: String,
        #[arg(short, long)]
        image: PathBuf,
        #[arg(short, long)]
        output: PathBuf,
    },
    Print {
        #[arg(short, long)]
        image: PathBuf,
    },
}

impl Command {
    #[allow(clippy::many_single_char_names)]
    fn execute(&self) -> Result<(), Error> {
        match self {
            Command::Hide {
                message,
                image,
                output,
            } => {
                let mut img = ImageReader::open(image)?.decode()?.to_rgba8();
                let mut message = message.bytes();

                for pixel in img.pixels_mut() {
                    let Some(b) = message.next() else {
                        *pixel = NULL_PIXEL;
                        break;
                    };
                    let image::Rgba(rgba) = pixel;
                    *pixel = image::Rgba(encode(*rgba, b));
                }

                img.save(output)?;
            }
            Command::Print { image } => {
                let img = ImageReader::open(image)?.decode()?.to_rgba8();
                let mut message = String::new();

                for pixel in img.pixels() {
                    if *pixel == NULL_PIXEL {
                        break;
                    }
                    let image::Rgba(rgba) = pixel;
                    message.push(decode(*rgba) as char);
                }
                println!("{message}");
            }
        }
        Ok(())
    }
}

fn encode(rgba: [u8; 4], byte: u8) -> [u8; 4] {
    let [r, g, b, a] = rgba;
    [
        (r & 0xFC) + ((byte & 0xC0) >> 6),
        (g & 0xFC) + ((byte & 0x30) >> 4),
        (b & 0xFC) + ((byte & 0x0C) >> 2),
        (a & 0xFC) + (byte & 0x03),
    ]
}

fn decode(rgba: [u8; 4]) -> u8 {
    let [r, g, b, a] = rgba;
    ((r & 0x03) << 6) + ((g & 0x03) << 4) + ((b & 0x03) << 2) + (a & 0x03)
}

fn main() -> Result<(), Error> {
    Args::parse().command.execute()
}

#[cfg(test)]
mod tests {
    #[test]
    fn encode_empty_pixel() {
        assert_eq!(super::encode([0, 0, 0, 0], 0b0000_0000), [0, 0, 0, 0]);
        assert_eq!(super::encode([0, 0, 0, 0], 0b0000_0001), [0, 0, 0, 1]);
        assert_eq!(super::encode([0, 0, 0, 0], 0b0000_0010), [0, 0, 0, 2]);
        assert_eq!(super::encode([0, 0, 0, 0], 0b0000_0100), [0, 0, 1, 0]);
        assert_eq!(super::encode([0, 0, 0, 0], 0b0000_1000), [0, 0, 2, 0]);
        assert_eq!(super::encode([0, 0, 0, 0], 0b0001_0000), [0, 1, 0, 0]);
        assert_eq!(super::encode([0, 0, 0, 0], 0b0010_0000), [0, 2, 0, 0]);
        assert_eq!(super::encode([0, 0, 0, 0], 0b0100_0000), [1, 0, 0, 0]);
        assert_eq!(super::encode([0, 0, 0, 0], 0b1000_0000), [2, 0, 0, 0]);
        assert_eq!(super::encode([0, 0, 0, 0], 0b1111_1111), [3, 3, 3, 3]);
    }

    #[test]
    fn decode() {
        assert_eq!(super::decode([0, 0, 0, 0]), 0);
        assert_eq!(super::decode([0, 0, 0, 1]), 1);
        assert_eq!(super::decode([0, 0, 0, 2]), 2);
        assert_eq!(super::decode([0, 0, 0, 3]), 3);
        assert_eq!(super::decode([0, 0, 0, 4]), 0);
        assert_eq!(super::decode([0, 0, 1, 0]), 4);
        assert_eq!(super::decode([0, 0, 2, 0]), 8);
        assert_eq!(super::decode([0, 0, 3, 0]), 12);
        assert_eq!(super::decode([0, 0, 4, 0]), 0);
        assert_eq!(super::decode([0, 1, 0, 0]), 16);
        assert_eq!(super::decode([0, 2, 0, 0]), 32);
        assert_eq!(super::decode([0, 3, 0, 0]), 48);
        assert_eq!(super::decode([0, 4, 0, 0]), 0);
        assert_eq!(super::decode([1, 0, 0, 0]), 64);
        assert_eq!(super::decode([2, 0, 0, 0]), 128);
        assert_eq!(super::decode([3, 0, 0, 0]), 192);
        assert_eq!(super::decode([4, 0, 0, 0]), 0);
    }
}
