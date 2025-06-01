use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

use blake2::{digest::consts::U32, Blake2b, Digest};
use chacha20poly1305::{aead::Aead as _, KeyInit as _, XChaCha20Poly1305};
use image::ImageReader;

use crate::{
    sekreto::{self, Dosiero, Sekreto},
    servajxoj,
};

const USIZE_LEN: usize = usize::BITS as usize / 8;

type Blake2b32 = Blake2b<U32>;

#[derive(Debug, thiserror::Error)]
pub enum Eraro {
    #[error("Io error: {0}")]
    Io(#[from] io::Error),
    #[error("Image error: {0}")]
    Image(#[from] image::error::ImageError),
    #[error("Sekreto error: {0}")]
    Sekreto(#[from] sekreto::Eraro),
}

#[derive(bincode::Encode, bincode::Decode)]
pub enum Cxifrado {
    Chacha20Poly1305,
}

impl Cxifrado {
    pub fn cxifri<S: AsRef<str>>(
        enhavo: &[u8],
        sekreta_frazo: S,
    ) -> Result<Vec<u8>, chacha20poly1305::Error> {
        XChaCha20Poly1305::new(Self::derivi_sxlosilo(sekreta_frazo)[..].into())
            .encrypt(vec![0; 24][..].into(), enhavo)
    }

    pub fn decxifri<S: AsRef<str>>(
        cxifrita: &[u8],
        sekreta_frazo: S,
    ) -> Result<Vec<u8>, chacha20poly1305::Error> {
        XChaCha20Poly1305::new(Self::derivi_sxlosilo(sekreta_frazo)[..].into())
            .decrypt(vec![0; 24][..].into(), cxifrita)
    }

    fn derivi_sxlosilo<S: AsRef<str>>(sekreta_frazo: S) -> chacha20poly1305::Key {
        let mut kdf = Blake2b32::new();
        kdf.update(sekreta_frazo.as_ref().as_bytes());
        kdf.finalize()
    }
}

pub fn ensxipigxi<P: AsRef<Path>>(
    kovrilodosiero: P,
    dosierenhavo: P,
    stegodosiero: P,
    cxifrado: Option<(String, String)>,
) -> Result<(), Eraro> {
    let mut bildo = ImageReader::open(kovrilodosiero)?.decode()?.to_rgba8();

    let dosiero = Dosiero::try_from(dosierenhavo.as_ref())?;

    let sekreto = if let Some((sugesto, sekreta_frazo)) = cxifrado {
        Sekreto::cxifrata(sugesto, sekreta_frazo, &dosiero)?
    } else {
        Sekreto::klara(dosiero)
    };

    let ensxipigxis = <&Sekreto as TryInto<Vec<u8>>>::try_into(&sekreto)?;
    let ensxipigxis = [ensxipigxis.len().to_le_bytes().to_vec(), ensxipigxis].concat();

    for (pikselo, b) in bildo.pixels_mut().zip(ensxipigxis.iter()) {
        let image::Rgba(rgba) = pikselo;
        *pikselo = image::Rgba(servajxoj::cxifri_bajton(*rgba, *b));
    }

    bildo.save(stegodosiero)?;
    Ok(())
}

pub fn ekstrakti<P: AsRef<Path>>(
    stegodosiero: P,
    celloko: Option<PathBuf>,
    sekreta_frazo: Option<String>,
) -> Result<(), Eraro> {
    let bildo = ImageReader::open(stegodosiero)?.decode()?.to_rgba8();
    let mut pikseloj = bildo.pixels();

    let ensxipigxis_lon = usize::from_le_bytes(
        <Vec<u8> as TryInto<[u8; USIZE_LEN]>>::try_into(
            (0..USIZE_LEN)
                .filter_map(|_| pikseloj.next())
                .map(|image::Rgba(rgba)| servajxoj::decxifri_bajton(*rgba))
                .collect::<Vec<u8>>(),
        )
        .unwrap(),
    );

    let Dosiero { nomo, enhavo } = Sekreto::try_from(
        pikseloj
            .zip(0..ensxipigxis_lon)
            .map(|(image::Rgba(rgba), _)| servajxoj::decxifri_bajton(*rgba))
            .collect::<Vec<_>>(),
    )?
    .into_dosiero(sekreta_frazo)?;

    fs::write(
        celloko
            .unwrap_or_else(|| env::current_dir().unwrap())
            .join(nomo),
        enhavo,
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    const MESAGXO: &str = "This tool has been created to share pirated stuff :)";
    const SEKRETA_FRAZO: &str = "If buying isn't owning, piracy isn't stealing.";

    #[test]
    fn cxifri() {
        let cxifrado_bajtoj = super::Cxifrado::cxifri(MESAGXO.as_bytes(), SEKRETA_FRAZO).unwrap();

        assert_ne!(cxifrado_bajtoj, MESAGXO.as_bytes());
        assert_eq!(
            cxifrado_bajtoj,
            vec![
                63, 92, 199, 64, 226, 33, 166, 38, 34, 177, 45, 247, 203, 153, 163, 209, 225, 177,
                65, 16, 245, 230, 208, 110, 220, 69, 140, 50, 211, 181, 89, 172, 86, 64, 55, 240,
                20, 129, 113, 131, 102, 226, 13, 130, 30, 126, 12, 229, 171, 41, 201, 202, 189,
                108, 23, 235, 139, 161, 0, 226, 165, 202, 44, 90, 172, 182, 20, 176
            ]
        );
    }

    #[test]
    fn decxifri() {
        assert_eq!(
            &super::Cxifrado::cxifri(
                &[
                    63, 92, 199, 64, 226, 33, 166, 38, 34, 177, 45, 247, 203, 153, 163, 209, 225,
                    177, 65, 16, 245, 230, 208, 110, 220, 69, 140, 50, 211, 181, 89, 172, 86, 64,
                    55, 240, 20, 129, 113, 131, 102, 226, 13, 130, 30, 126, 12, 229, 171, 41, 201,
                    202, 189, 108, 23, 235, 139, 161, 0, 226, 165, 202, 44, 90, 172, 182, 20, 176
                ],
                SEKRETA_FRAZO
            )
            .unwrap()[..MESAGXO.len()],
            MESAGXO.as_bytes()
        );
    }
}
