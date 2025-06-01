use std::{fs, io, path::Path};

use crate::komuna;

#[derive(Debug, thiserror::Error)]
pub enum Eraro {
    #[error("Chacha20Poly1305 decrypting error: {0}")]
    Chacha20Poly1305Decrypting(chacha20poly1305::Error),
    #[error("Bincode encoding error: {0}")]
    BincodeEncoding(#[from] bincode::error::EncodeError),
    #[error("Bincode decoding error: {0}")]
    BincodeDecoding(#[from] bincode::error::DecodeError),
    #[error("Missing passphrase, hint: {0}")]
    SekretoFrazo(String),
}

#[derive(bincode::Encode, bincode::Decode)]
pub struct Dosiero {
    pub nomo: String,
    // pub kunpremo: Option<komuna::Kunpremo>,
    pub enhavo: Vec<u8>,
}

impl TryFrom<&Dosiero> for Vec<u8> {
    type Error = bincode::error::EncodeError;

    fn try_from(valoro: &Dosiero) -> Result<Self, Self::Error> {
        bincode::encode_to_vec(valoro, Sekreto::bincode_agordoj())
    }
}

impl TryFrom<Vec<u8>> for Dosiero {
    type Error = bincode::error::DecodeError;

    fn try_from(valoro: Vec<u8>) -> Result<Self, Self::Error> {
        let (tio, _) = bincode::decode_from_slice(&valoro, Sekreto::bincode_agordoj())?;
        Ok(tio)
    }
}

#[derive(bincode::Encode, bincode::Decode)]
pub enum Sekreto {
    Klara(Dosiero),
    Cxifrata {
        sugesto: String,
        cxifrata_dosiero: Vec<u8>,
    },
}

impl Sekreto {
    pub fn klara(dosiero: Dosiero) -> Self {
        Self::Klara(dosiero)
    }

    pub fn cxifrata<S: AsRef<str>>(
        sugesto: S,
        sekreta_frazo: S,
        dosiero: &Dosiero,
    ) -> Result<Self, Eraro> {
        Ok(Self::Cxifrata {
            sugesto: sugesto.as_ref().to_string(),
            cxifrata_dosiero: komuna::Cxifrado::cxifri(
                &<&Dosiero as TryInto<Vec<u8>>>::try_into(dosiero)?,
                sekreta_frazo,
            )
            .map_err(Eraro::Chacha20Poly1305Decrypting)?,
        })
    }

    pub fn into_dosiero(self, sekreta_frazo: Option<String>) -> Result<Dosiero, Eraro> {
        Ok(match (self, sekreta_frazo) {
            (Sekreto::Klara(dosiero), _) => dosiero,
            (
                Sekreto::Cxifrata {
                    cxifrata_dosiero, ..
                },
                Some(sekreta_frazo),
            ) => Dosiero::try_from(
                komuna::Cxifrado::decxifri(&cxifrata_dosiero, sekreta_frazo)
                    .map_err(Eraro::Chacha20Poly1305Decrypting)?,
            )?,
            (Sekreto::Cxifrata { sugesto, .. }, None) => Err(Eraro::SekretoFrazo(sugesto))?,
        })
    }

    const fn bincode_agordoj() -> bincode::config::Configuration {
        bincode::config::standard()
            .with_little_endian()
            .with_variable_int_encoding()
    }
}

impl TryFrom<&Path> for Dosiero {
    type Error = io::Error;

    fn try_from(valoro: &Path) -> Result<Self, Self::Error> {
        Ok(Self {
            nomo: valoro.file_name().unwrap().to_str().unwrap().to_string(),
            enhavo: fs::read(valoro)?,
        })
    }
}

impl TryFrom<&Sekreto> for Vec<u8> {
    type Error = Eraro;

    fn try_from(valoro: &Sekreto) -> Result<Self, Self::Error> {
        Ok(bincode::encode_to_vec(valoro, Sekreto::bincode_agordoj())?)
    }
}

impl TryFrom<Vec<u8>> for Sekreto {
    type Error = Eraro;

    fn try_from(valoro: Vec<u8>) -> Result<Self, Self::Error> {
        let (tio, _) = bincode::decode_from_slice(&valoro, Sekreto::bincode_agordoj())?;
        Ok(tio)
    }
}
