use std::{
    env, fs,
    path::{Path, PathBuf},
};

use image::ImageReader;

use crate::{eraro::Eraro, sekreto::Sekreto, servajxoj};

const USIZE_LEN: usize = usize::BITS as usize / 8;

pub fn ensxipigxi<P: AsRef<Path>>(
    kovrilodosiero: P,
    dosierenhavo: P,
    stegodosiero: P,
) -> Result<(), Eraro> {
    let mut bildo = ImageReader::open(kovrilodosiero)?.decode()?.to_rgba8();
    let sekreto = Sekreto::try_from(dosierenhavo.as_ref())?;
    let ensxipigxis = <&Sekreto as TryInto<Vec<u8>>>::try_into(&sekreto)?;
    let ensxipigxis = [ensxipigxis.len().to_le_bytes().to_vec(), ensxipigxis].concat();

    for (pikselo, b) in bildo.pixels_mut().zip(ensxipigxis.iter()) {
        let image::Rgba(rgba) = pikselo;
        *pikselo = image::Rgba(servajxoj::cxifri_bajton(*rgba, *b));
    }

    bildo.save(stegodosiero)?;
    Ok(())
}

pub fn ekstrakti<P: AsRef<Path>>(stegodosiero: P, celloko: Option<PathBuf>) -> Result<(), Eraro> {
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

    let Sekreto { dosiero, enhavo } = Sekreto::try_from(
        pikseloj
            .zip(0..ensxipigxis_lon)
            .map(|(image::Rgba(rgba), _)| servajxoj::decxifri_bajton(*rgba))
            .collect::<Vec<_>>(),
    )?;

    fs::write(
        celloko
            .unwrap_or_else(|| env::current_dir().unwrap())
            .join(dosiero),
        enhavo,
    )?;

    Ok(())
}
