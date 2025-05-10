use std::{env, fs, io, os::unix::ffi::OsStrExt, path::PathBuf};

use clap::Parser;
use image::ImageReader;

const USIZE_LEN: usize = usize::BITS as usize / 8;
// const NULL_PIXEL: image::Rgba<u8> = image::Rgba([0, 0, 0, 0]);

// https://linuxcommandlibrary.com/man/steghide
// https://jsmnbom.github.io/toki-pona-helper/sitelenPonaConverter.html

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Io error: {0}")]
    Io(#[from] io::Error),
    #[error("Image error: {0}")]
    Image(#[from] image::error::ImageError),
    // #[error("Bincode encoding error: {0}")]
    // BincodeEncoding(#[from] bincode::error::EncodeError),
}

/// Smaragdina
#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Parser)]
enum Command {
    Embed {
        /// Path to the cover file in which to embed data
        #[arg(short, long)]
        cover_file: PathBuf,
        /// Path to the data file to be hidden
        #[arg(short, long)]
        embed_file: PathBuf,
        /// Path to the stego file (containing hidden data) for extraction
        #[arg(short, long)]
        stego_file: PathBuf,
    },
    Extract {
        /// Path to the stego file (containing hidden data) for extraction
        #[arg(short, long)]
        stego_file: PathBuf,
        /// Path to directory where the embed tile will be extracted
        #[arg(short, long)]
        destination: Option<PathBuf>,
    },
}

impl Command {
    fn execute(self) -> Result<(), Error> {
        match self {
            Command::Embed {
                cover_file,
                embed_file,
                stego_file,
            } => {
                let mut img = ImageReader::open(cover_file)?.decode()?.to_rgba8();
                // let mut sekreto = <&Sekreto as TryInto<Vec<u8>>>::try_into(&Sekreto::try_from(
                //     embed_file.as_path(),
                // )?)?;
                // sekreto.push(0);
                let embed = fs::read(&embed_file)?;
                let embed = [
                    embed.len().to_be_bytes().to_vec(),
                    [embed_file.file_name().unwrap().as_bytes(), &[0]].concat(),
                    embed,
                ]
                .concat();

                for (pixel, b) in img.pixels_mut().zip(embed.iter()) {
                    let image::Rgba(rgba) = pixel;
                    *pixel = image::Rgba(encode(*rgba, *b));
                }

                img.save(stego_file)?;
            }
            Command::Extract {
                stego_file,
                destination,
            } => {
                let img = ImageReader::open(stego_file)?.decode()?.to_rgba8();
                let mut embed = Vec::new();

                let mut pixels = img.pixels();
                let embed_len = usize::from_be_bytes(
                    <Vec<u8> as TryInto<[u8; USIZE_LEN]>>::try_into(
                        (0..USIZE_LEN)
                            .filter_map(|_| pixels.next())
                            .map(|image::Rgba(rgba)| decode(*rgba))
                            .collect::<Vec<u8>>(),
                    )
                    .unwrap(),
                );

                let mut embed_file = String::new();
                loop {
                    let image::Rgba(rgba) = pixels
                        .next()
                        .expect("Failed to read the name of the embed file");
                    let b = decode(*rgba);
                    if b == 0 {
                        break;
                    }
                    embed_file.push(decode(*rgba) as char);
                }

                for (pixel, _) in pixels.zip(0..embed_len) {
                    let image::Rgba(rgba) = pixel;
                    embed.push(decode(*rgba));
                }

                fs::write(
                    if let Some(destination) = destination {
                        destination
                    } else {
                        env::current_dir().unwrap()
                    }
                    .join(embed_file),
                    embed,
                )?;
            }
        }
        Ok(())
    }
}

// #[derive(bincode::Encode, bincode::Decode)]
// struct Sekreto {
//     r#type: FileType,
//     name: String,
//     content: Vec<u8>,
// }

// #[derive(bincode::Encode, bincode::Decode)]
// enum FileType {
//     GrayscaleImage((u32, u32)),
//     Raw,
// }

// impl TryFrom<&Path> for Sekreto {
//     type Error = Error;

//     fn try_from(value: &Path) -> Result<Self, Self::Error> {
//         Ok(match ImageReader::open(value)?.decode() {
//             Ok(image) => Self {
//                 r#type: FileType::GrayscaleImage((0, 0)),
//                 name: value.file_name().unwrap().to_str().unwrap().to_string(),
//                 content: image.grayscale().as_bytes().to_vec(),
//             },
//             Err(_) => Self {
//                 r#type: FileType::Raw,
//                 name: value.file_name().unwrap().to_str().unwrap().to_string(),
//                 content: fs::read(value)?,
//             },
//         })
//     }
// }

// impl TryFrom<&Sekreto> for Vec<u8> {
//     type Error = bincode::error::EncodeError;

//     fn try_from(value: &Sekreto) -> Result<Self, Self::Error> {
//         bincode::encode_to_vec(
//             value,
//             bincode::config::standard()
//                 .with_big_endian()
//                 .with_fixed_int_encoding(),
//         )
//     }
// }

// impl TryFrom<Vec<u8>> for Sekreto {
//     type Error = bincode::error::DecodeError;

//     fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
//         let (this, _) = bincode::decode_from_slice(
//             &value,
//             bincode::config::standard()
//                 .with_big_endian()
//                 .with_fixed_int_encoding(),
//         )?;
//         Ok(this)
//     }
// }

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
